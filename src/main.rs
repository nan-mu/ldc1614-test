//! 自动化测试ldc1614

mod channel;
mod config;
mod handler;
mod motor;

use clap::Parser;
use log::{error, info};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 日志等级
    #[clap(short, long, default_value = "debug")]
    log_level: String,

    /// 配置文件路径
    #[clap(short, long, default_value = "tasks.yml")]
    config: String,
}

use std::{sync, time::Duration};

#[derive(Debug, Clone)]
struct Record {
    /// 时间戳
    timestamp: chrono::DateTime<chrono::Local>,
    /// 数据
    data: u32,
    /// 通道
    channel: ldc1614::Channel,
    /// 可选的标记
    mark: Option<sync::Arc<str>>,
}

use rppal::gpio;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("gpio使用错误: {0}")]
    Gpio(#[from] gpio::Error),
    #[error("ldc1614错误")]
    Ldc1614(ldc1614::Error),
    #[error("生产者发现通道已被关闭")]
    ProducerError,
    #[error("生产者无法获得i2c和ldc互斥锁")]
    ProducerJoinError,
    #[error("配置文件填写错误")]
    ConfigError,
    #[error("数据库错误: {0}")]
    Database(#[from] fred::error::RedisError),
}

use tokio::{sync::broadcast, time};
impl From<broadcast::error::SendError<Record>> for Error {
    fn from(_value: broadcast::error::SendError<Record>) -> Self {
        Error::ProducerJoinError
    }
}

type Result<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 初始化日志
    use env_logger::Builder;
    use log::debug;
    use std::str::FromStr;
    Builder::from_default_env()
        .filter(
            None,
            log::LevelFilter::from_str(&args.log_level.to_uppercase())
                .unwrap_or(log::LevelFilter::Debug),
        )
        .init();

    debug!("读取配置文件");
    use config::Config;
    let config = Config::read_config(&args.config).unwrap();

    debug!("初始化i2c设备");
    use ldc1614::{bitmap::MANUFCTURER_ID, Ldc};
    use rppal::i2c::I2c;
    use tokio::sync::Mutex;
    let mut i2c: I2c = I2c::new().unwrap();
    let ldc = Ldc::<0x2b>::new(&mut i2c);
    let manufcturer_id = ldc
        .register
        .manufcturer_id
        .read(&mut i2c, MANUFCTURER_ID::manufcturer_id);
    debug!("制造商id: {}", manufcturer_id);
    let ldc = Arc::new(Mutex::new(ldc));
    let i2c = Arc::new(Mutex::new(i2c));

    debug!("初始化gpio");
    use motor::Motor;
    use rppal::gpio::Gpio;
    let gpio = Gpio::new()
        .map_err(|e| {
            error!(
                "GPIO 初始化失败: {:?}。考虑运行：`sudo chown $USER /dev/*`",
                e
            );
            std::process::exit(1);
        })
        .unwrap();
    let pwm = gpio.get(21).unwrap().into_output_low();
    let dir = gpio.get(12).unwrap().into_output_high();
    let mut motor = Motor::new(pwm, dir);
    info!("电机初始位置为 {}mm", config.tasks[0].position()[0]);
    motor.set_position(config.tasks[0].position()[0]);

    debug!("创建广播通道");
    use handler::Consumer;
    use std::{path::Path, sync::Arc};
    use tokio::sync::broadcast;
    let (tx, _) = broadcast::channel(512);

    let mut rx = vec![];
    if let Some(csv) = config.csv {
        debug!("发现csv配置");
        rx.push((
            Consumer::Csv {
                path: csv.path.map(|p| Arc::from(Path::new(&p))),
            },
            tx.subscribe(),
        ));
    }

    if let Some(redis) = config.redis {
        debug!("发现redis配置");
        rx.push((
            Consumer::Redis {
                url: redis.url.into(),
            },
            tx.subscribe(),
        ));
    }

    handler::Handler { rx }.submit().await.unwrap();

    debug!("开始进行测试");
    for task in config.tasks {
        for postion in task.position() {
            // 启动电机
            let moter_ok = motor.goto(postion);

            let mut channel =
                channel::Channel::from(task.channel(), tx.clone(), ldc.clone(), i2c.clone());
            let register = task.registers.clone().unwrap();
            use std::collections::HashMap;

            // 提取特殊设置字符串
            let mut settlecount = String::new();
            let mut rcount = String::new();
            let mut register: HashMap<String, u16> = register
                .into_iter()
                .filter_map(|(field, value)| match value.parse() {
                    Ok(value) => Some((field.to_uppercase(), value)),
                    Err(_) => {
                        match field.to_uppercase().as_str() {
                            "SETTLECOUNT" => {
                                settlecount = value;
                            }
                            "RCOUNT" => {
                                rcount = value;
                            }
                            _ => {
                                error!("无法对 {} 进行批量测试", field);
                                panic!()
                            }
                        }
                        None
                    }
                })
                .collect();

            // 字符串转range
            let settlecount = config::matlab_type_range::<u16, _>(&settlecount);
            let settlecount = (settlecount.0..settlecount.2).step_by(settlecount.1 as usize);
            let rcount = config::matlab_type_range::<u16, _>(&rcount);
            let rcount_range = (rcount.0..rcount.2).step_by(rcount.1 as usize);

            // 等到电机就位
            moter_ok.await.unwrap();

            // 拼接寄存器并拉取数据
            for settlecount in settlecount {
                let _ = (&mut register).insert("SETTLECOUNT".to_string(), settlecount);
                for rcount in rcount_range.clone() {
                    let _ = (&mut register).insert("RCOUNT".to_string(), rcount);
                    channel.apply_reg_config(&register).await.unwrap();
                    let mark: Arc<str> = Arc::from(format!(
                        "\"postion:{postion},settlecount:{settlecount},rcount:{rcount}\""
                    ));
                    for times in 0..task.count {
                        time::sleep(Duration::from_millis((settlecount / 10) as u64)).await;
                        match channel.submit(Some(mark.clone())).await {
                            Ok(_) => debug!("测量成功"),
                            Err(e) => error!("postion:{postion},settlecount:{settlecount},rcount:{rcount}，测量第 {} 次失败: {:?}", times, e),
                        };
                    }
                }
            }
            info!("测试任务完成，电机正在归位");
            motor.goto(0.0).await.unwrap();
        }
    }

    info!("测试任务完成，电机正在归位");
    motor.goto(0.0).await.unwrap();
    Ok(())
}
