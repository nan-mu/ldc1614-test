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
}

fn generate_pwm_signal(
    frequency: f64,
    duty_cycle: f64,
    microstepping: u8,
    basic_step_angle: f64,
    distance_per_rotation: f64,
    travel_distance: f64, // 滑轨前进的距离（毫米）
) {
    use rppal::pwm::{Channel as pwm_channel, Polarity, Pwm};
    use std::{thread, time::Duration};

    // 初始化 PWM 通道
    let pwm = Pwm::with_frequency(
        pwm_channel::Pwm0,
        frequency,
        duty_cycle,
        Polarity::Normal,
        true,
    )
    .expect("Failed to initialize PWM");

    // 启动 PWM 输出
    pwm.enable().expect("Failed to enable PWM");

    // 计算速度 (mm/s)
    let speed =
        frequency * distance_per_rotation * basic_step_angle / (microstepping as f64 * 360.0);

    // 计算滑轨前进指定距离需要的时间 (秒)
    let travel_time = travel_distance / speed;

    // 使滑轨前进指定距离
    thread::sleep(Duration::from_secs_f64(travel_time));

    // 停止 PWM 输出
    pwm.disable().expect("Failed to disable PWM");
}

#[tokio::main]
async fn main() {
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

    let frequency = 1000.0;
    let duty_cycle = 0.5;
    let microstepping = 16;
    const BASIC_STEP_ANGLE: f64 = 1.8;
    const DISTANCE_PER_ROTATION: f64 = 1.0; // 一圈移动的距离，单位：mm
    let travel_distance = 10.0; // 让滑轨前进10毫米

    // 调用生成 PWM 信号的函数
    generate_pwm_signal(
        frequency,
        duty_cycle,
        microstepping,
        BASIC_STEP_ANGLE,
        DISTANCE_PER_ROTATION,
        travel_distance,
    );

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
}
