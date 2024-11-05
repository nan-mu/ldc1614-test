//! 用于测试ldc161x板子能否正常工作

use std::{thread::sleep, time::Duration};

fn main() {
    // 初始化日志
    use env_logger::Builder;
    use log::debug;
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    use rppal::i2c::I2c;
    debug!("初始化i2c设备");
    let mut i2c = I2c::new().unwrap();
    let ldc = Ldc::<0x2b>::new(&mut i2c);

    loop {
        sleep(Duration::from_secs(1));
        println!(
            "{}",
            ldc.read_data(&mut i2c, ldc1614::Channel::Zero).unwrap()
        )
    }
}
