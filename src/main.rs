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

fn main() {
    // 解析命令行参数
    let args = Args::parse();
    assert!(args.channel <= 3, "错误：通道只能选择0, 1, 2, 3");
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
    // let mut i2c = I2c::new().unwrap();
    // let ldc = Ldc::<0x2b>::new(&mut i2c);
    // ldc.defaule_config(&mut i2c, args.channel).unwrap();

    loop {
        use std::{thread, time::Duration};
        thread::sleep(Duration::from_millis(args.sample_time));
        // println!(
        //     "{}",
        //     // ldc.read_data(&mut i2c, args.channel).unwrap()
        // )
    }
}
