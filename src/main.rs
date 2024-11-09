//! 自动化测试ldc1614

mod channel;
mod database;
mod handler;

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 采样时间（毫秒）
    #[clap(short, long, default_value_t = 1000)]
    sample_time: u64,

    /// 日志等级
    #[clap(short, long, default_value = "debug")]
    log_level: String,

    /// 选择的通道（0-3）
    #[clap(short, long, default_value_t = 0)]
    channel: u8,

    /// 单次运行完成
    #[clap(short, long, default_value_t = 5)]
    test_count: u8,
}

use std::sync;

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
struct Motor {
    pwm: gpio::OutputPin,
    dir: gpio::OutputPin,
}

use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("gpio使用错误: {0}")]
    Gpio(#[from] gpio::Error),
    #[error("ldc1614错误")]
    Ldc1614(ldc1614::Error),
    #[error("生产者发现通道已被关闭")]
    ProducerError,
    #[error("生产者无法获得i2c和ldc互斥锁")]
    ProducerJoinError,
    #[error("配置文件填写错误")]
    ConfigErr,
    #[error("数据库错误: {0}")]
    Database(#[from] fred::error::RedisError),
}

use tokio::sync::broadcast;
impl From<broadcast::error::SendError<Record>> for Error {
    fn from(_value: broadcast::error::SendError<Record>) -> Self {
        Error::ProducerJoinError
    }
}

type Result<T> = core::result::Result<T, Error>;

impl Motor {
    /// GPIO模拟PWM输出
    /// 首先说明电机的连接方式是共阴极连接，
    /// ENA-、DIR-、PUL-接控制器的地，
    /// ENA+接使能信号，DIR+接方向信号物理口32，PUL+接脉冲信号物理口40
    /// * distance 电机运动距离（正数为正向移动）
    async fn moving(&mut self, distance: f64) -> Result<()> {
        use tokio::time::{self, Duration};
        // 电机运动速度 (mm/s)= 脉冲频率 * 丝杆导程 * 电机旋转步长 / (编码器细分度 * 360°)
        // > 滑轨每转的距离是1.0mm，步进电机在没有细分的情况下，电机每步进一次时的角度为1.8°
        const SPEED: f64 = 1.5625;
        // 启动 PWM 输出
        self.pwm.set_pwm_frequency(5000.0, 0.5)?;
        //设置方向引脚
        match distance.is_sign_negative() {
            true => {
                self.dir.set_low();
            }
            false => {
                self.dir.set_high();
            }
        }
        // 异步等待滑轨前进指定距离
        time::sleep(Duration::from_secs_f64(distance.abs() / SPEED)).await;
        // 停止 PWM 输出
        self.pwm.clear_pwm()?;
        Ok(())
    }
}

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

    debug!("初始化参数");
    use ldc1614::Channel;
    assert!(args.channel <= 3, "错误：通道只能选择0, 1, 2, 3");
    let real_channel = match args.channel {
        0 => Channel::Zero,
        1 => Channel::One,
        2 => Channel::Two,
        3 => Channel::Three,
        _ => {
            assert!(args.channel <= 3, "错误：通道只能选择0, 1, 2, 3");
            std::process::exit(1);
        }
    };

    debug!("初始化i2c设备");
    use rppal::i2c::I2c;
    let mut i2c: I2c = I2c::new().unwrap();
    use ldc1614::Ldc;
    let ldc = Ldc::<0x2b>::new(&mut i2c);
    ldc.defaule_config(&mut i2c, real_channel).unwrap();

    debug!("初始化gpio");
    use rppal::gpio::Gpio;
    let gpio = Gpio::new().unwrap();
    let pwm = gpio.get(21).unwrap().into_output_low();
    let dir = gpio.get(12).unwrap().into_output_high();
    let mut motor = Motor { pwm, dir };

    // 调用生成 PWM 信号的函数
    //频率为1000hz，占空比50%，细分度16，让滑轨前进10mm
    motor.moving(10.0).await.unwrap();

    loop {
        // use std::io::BufRead;
        // let mut input = String::new();
        // match handle.read_line(&mut input) {
        //     Err(e) => {
        //         error!("错误的输入: {}", e);
        //         continue;
        //     }
        //     _ => {}
        // }
        // let mark = match input.len() {
        //     0 => None,
        //     _ => Some(input),
        // };

        // for _ in 0..args.test_count {
        //     use std::{thread, time::Duration};
        //     thread::sleep(Duration::from_millis(args.sample_time));

        //     let data = ldc.read_data(&mut i2c, real_channel).unwrap();
        //     // 写入数据行
        //     wtr.serialize(Record {
        //         timestamp: Local::now(),
        //         data,
        //         mark: Cow::Borrowed(mark),
        //     })
        //     .unwrap();
        // }
        // wtr.flush().unwrap();

        // info!(
        //     "数据写入完成到 {}，完成时间 {}{}",
        //     filename,
        //     Local::now().to_rfc3339(),
        //     match mark {
        //         None => "".to_string(),
        //         Some(_) => format!("，标记为 {}", mark.unwrap()),
        //     }
        // );
    }
}
