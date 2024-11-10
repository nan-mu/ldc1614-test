//! 自动化测试ldc1614

mod channel;
mod config;
mod handler;
mod motor;

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

use tokio::sync::broadcast;
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

    debug!("读取配置文件");
    use config::Config;
    let config = Config::read_config("tasks.yml").unwrap();

    debug!("初始化i2c设备");
    use rppal::i2c::I2c;
    let mut i2c: I2c = I2c::new().unwrap();
    use ldc1614::Ldc;
    let ldc = Ldc::<0x2b>::new(&mut i2c);
    ldc.defaule_config(&mut i2c, real_channel).unwrap();

    debug!("初始化gpio");
    use motor::Motor;
    use rppal::gpio::Gpio;
    let gpio = Gpio::new().unwrap();
    let pwm = gpio.get(21).unwrap().into_output_low();
    let dir = gpio.get(12).unwrap().into_output_high();
    let mut motor = Motor::new(pwm, dir);

    // //计划把主函数的loop中的代码改为：
    // //执行实验
    // let test_count: u32;
    // let test_location: Vec<f64>;
    // match config {
    //     Ok(config) => {
    //         test_count = config.test_count;
    //         test_location = config.test_location;
    //         // 使用 test_count 和 test_location
    //     }
    //     Err(e) => {
    //         eprintln!("配置文件读取失败: {}", e);
    //         test_location = Vec::new();
    //         test_count = 0;
    //         // 可以选择退出或执行其他处理逻辑
    //     }
    // }

    // for distance in test_location.iter() {
    //     //调用配置寄存器的函数

    //     //完成实验并写入实验数据
    //     for i in 0..test_count {
    //         // 调用电机移动函数
    //         // 调用生成 PWM 信号的函数
    //         // 频率为 1000hz，占空比 50%，细分度 16，让滑轨前进距离distance
    //         motor.moving(*distance).await.unwrap();

    //         // 将结果写入文件函数
    //         use std::{thread, time::Duration};
    //         thread::sleep(Duration::from_millis(args.sample_time));
    //         let data = ldc.read_data(&mut i2c, real_channel).unwrap();
    //         // 获取当前时间戳
    //         let timestamp = Local::now().to_string();
    //         // 写入数据行
    //         wtr.write_record(&[timestamp, data.to_string()]).unwrap();
    //         wtr.flush().unwrap();
    //         println!("Data has been written to {}", filename);

    //         // 调用电机移动函数，将电机移回原位
    //         motor.moving(-(*distance)).await.unwrap();
    //     }
    // }

    Ok(())
}

// loop {
//     use std::io::BufRead;
//     let mut input = String::new();
//     match handle.read_line(&mut input) {
//         Err(e) => {
//             error!("错误的输入: {}", e);
//             continue;
//         }
//         _ => {}
//     }
//     let mark = match input.len() {
//         0 => None,
//         _ => Some(input),
//     };

//     for _ in 0..args.test_count {
//         use std::{thread, time::Duration};
//         thread::sleep(Duration::from_millis(args.sample_time));

//         let data = ldc.read_data(&mut i2c, real_channel).unwrap();
//         // 写入数据行
//         wtr.serialize(Record {
//             timestamp: Local::now(),
//             data,
//             mark: mark.clone(),
//         })
//         .unwrap();
//     }
//     wtr.flush().unwrap();

//     info!(
//         "数据写入完成到 {}，完成时间 {}{}",
//         filename,
//         Local::now().to_rfc3339(),
//         match mark {
//             None => "".to_string(),
//             Some(_) => format!("，标记为 {}", mark.unwrap()),
//         }
//     );
// }
