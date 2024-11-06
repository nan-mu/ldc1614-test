//! 用于测试ldc161x板子能否正常工作

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

#[derive(serde::Serialize, Debug)]
struct Record {
    /// 时间戳
    timestamp: chrono::DateTime<chrono::Local>,
    /// 数据
    data: u32,
    /// 可选的标记
    mark: Option<String>,
}

#[tokio::main]
async fn main() {
    use ldc1614::Channel;
    let args = Args::parse();

    // 初始化日志
    use env_logger::Builder;
    use log::{debug, error};
    use std::str::FromStr;
    Builder::from_default_env()
        .filter(
            None,
            log::LevelFilter::from_str(&args.log_level.to_uppercase())
                .unwrap_or(log::LevelFilter::Debug),
        )
        .init();

    debug!("初始化参数");
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
    let filename = format!(
        "data_{}.csv",
        Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    );

    // 创建并打开 CSV 文件
    let file = File::create(&filename).unwrap();

    // 创建 CSV 写入器
    let mut wtr = Writer::from_writer(file);

    // 写入表头
    wtr.write_record(&["timestamp", "data", "mark"]).unwrap();
    use std::io;
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        use std::io::BufRead;
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Err(e) => {
                error!("错误的输入: {}", e);
                continue;
            }
            _ => {}
        }
        let mark = match input.len() {
            0 => None,
            _ => Some(input),
        };

        for _ in 0..10 {
            use std::{thread, time::Duration};
            thread::sleep(Duration::from_millis(args.sample_time));

            let data = ldc.read_data(&mut i2c, real_channel).unwrap();
            // 写入数据行
            wtr.serialize(Record {
                timestamp: Local::now(),
                data,
                mark: mark.clone(),
            })
            .unwrap();
        }
        wtr.flush().unwrap();

        println!(
            "数据写入完成到 {}，完成时间 {}{}",
            filename,
            Local::now().to_rfc3339(),
            match mark {
                None => "".to_string(),
                Some(_) => format!("，标记为 {}", mark.unwrap()),
            }
        );
    }
}
