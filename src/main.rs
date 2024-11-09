use clap::Parser;
use rppal::gpio::{self, Gpio, OutputPin};
use tokio::sync::TryAcquireError;

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
}

struct Motor {
    pwm: OutputPin,
    dir: OutputPin,
}

#[derive(Debug)]
enum Error {
    Gpio(gpio::Error),
}

impl From<gpio::Error> for Error {
    fn from(value: gpio::Error) -> Self {
        Error::Gpio(value)
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
    // 解析命令行参数
    let args = Args::parse();
    assert!(args.channel <= 3, "错误：通道只能选择0, 1, 2, 3");

    use ldc1614::Channel;
    let real_channel: Channel;
    match args.channel {
        0 => real_channel = Channel::Zero,
        1 => real_channel = Channel::One,
        2 => real_channel = Channel::Two,
        3 => real_channel = Channel::Three,
        4_u8..=u8::MAX => todo!(),
    }

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

    debug!("初始化i2c设备");
    use rppal::i2c::I2c;
    let mut i2c = I2c::new().unwrap();
    use ldc1614::Ldc;
    let ldc = Ldc::<0x2b>::new(&mut i2c);
    ldc.defaule_config(&mut i2c, real_channel).unwrap();

    debug!("初始化gpio");
    let gpio = Gpio::new().unwrap();
    let pwm = gpio.get(21).unwrap().into_output_low();
    let dir = gpio.get(12).unwrap().into_output_high();
    let mut motor = Motor { pwm, dir };

    // debug!("连接数据库");
    // let client = redis::Client::open("redis://:mypassword@127.0.0.1/").unwrap();
    // let mut connect = client.get_multiplexed_tokio_connection().await.unwrap();

    //用于新建文件的库
    use chrono::Local;
    use csv::Writer;
    use std::fs::File;
    // 获取当前时间并格式化为文件名
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("data_{}.csv", timestamp);

    // 创建并打开 CSV 文件
    let file = File::create(&filename).unwrap();

    // 创建 CSV 写入器
    let mut wtr = Writer::from_writer(file);

    // 写入表头
    wtr.write_record(&["timestamp", "data"]).unwrap();

    // 调用生成 PWM 信号的函数
    //频率为1000hz，占空比50%，细分度16，让滑轨前进10mm
    motor.moving(10.0).await.unwrap();

    /*
    loop {
        use std::{thread, time::Duration};
        thread::sleep(Duration::from_millis(args.sample_time));

        let data = ldc.read_data(&mut i2c, real_channel).unwrap();

        // 获取当前时间戳
        let timestamp = Local::now().to_string();
        // 写入数据行
        wtr.write_record(&[timestamp, data.to_string()]).unwrap();
        wtr.flush().unwrap();
        println!("Data has been written to {}", filename);
        // println!(
        //     "{}",
        //     // ldc.read_data(&mut i2c, args.channel).unwrap()
        // )
    }
    */
    Ok(())
}
