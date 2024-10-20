
// i2c_ds3231.rs - Sets and retrieves the time on a Maxim Integrated DS3231
// RTC using I2C.

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::{self, I2c};
use ldc1x1x as ldc;

//ADDR接地时ldc1614的地址
const ADDR_LDC1614: u16 = 0x2A;

// Helper functions to encode and decode binary-coded decimal (BCD) values.
fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
}

fn dec2bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

fn main() {

    let mut  i2c=I2c::new()?;

    i2c.set_slave_address(ADDR_LDC1614)?;

    //创建实例，初始化LDC设备
    let mut ldc = ldc::Ldc::new(i2c,ADDR_LDC1614);

    ldc.set_sleep_mode(true).unwrap();

    ldc.reset().unwrap();

    //计算分频，这里需要理论计算
    let div = ldc::Fsensor::from_inductance_capacitance(12.583, 100.0).to_clock_dividers(None);
    
     //配置传感器
    for ch in [ldc::Channel::Zero] {



        //为指定通道设置时钟分频器，这一参数控制采样速度，影响数据读取的频率。
        //ldc.set_clock_dividers(ch, div).unwrap();
        ldc.set_clock_dividers(ch,div).unwrap();

        //设置转换稳定时间（settling time）为 40 个时钟周期。等待一段时间以确保信号稳定。
        ldc.set_conv_settling_time(ch, 40).unwrap();

        //设置参考计数转换间隔
        ldc.set_ref_count_conv_interval(ch, 0x0546).unwrap();

        //设置传感器驱动电流。这里的 0b01110 是一个二进制值，表示所需的电流设置
        ldc.set_sensor_drive_current(ch, 0b01110).unwrap();
    }

    //配置 LDC1614 设备的多路复用器，使用单通道时with_auto_scan值为false
    //设置去抖动滤波器带宽为 3.3 MHz
    ldc.set_mux_config(
        ldc::MuxConfig::default()
            .with_auto_scan(false)
            .with_deglitch_filter_bandwidth(ldc::Deglitch::ThreePointThreeMHz)

    )
    .unwrap();
    ldc.set_config(ldc::Config::default()).unwrap();
    ldc.set_error_config(
        ldc::ErrorConfig::default().with_amplitude_high_error_to_data_register(true),
    )
    .unwrap();

    ldc.set_sleep_mode(true).unwrap();

    // timing ignored because polling with a cp2112 with no delays is slow enough already
    // outputting just newline separated numbers so you can feed it into https://github.com/mogenson/ploot
    loop {
        println!(
            "{}",
            ldc.read_data_24bit(ldc::Channel::Zero).unwrap(),
        );
    }
}
