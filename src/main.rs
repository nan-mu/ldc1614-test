//! 用于测试ldc161x板子能否正常工作

//ADDR接地时ldc1614的地址
const ADDR_LDC1614: u8 = 0x2A;

fn main() {
    // use ldc1x1x::{self, Channel, Config, ErrorConfig, Fsensor, Ldc, MuxConfig};
    // use rppal::i2c::I2c;

    // let mut i2c = I2c::new().unwrap(); //创建i2c实例
    // let mut ldc = Ldc::new(&mut i2c, ADDR_LDC1614);

    // // ldc.reset().unwrap();

    // //计算分频，这里需要理论计算
    // let div = Fsensor::from_inductance_capacitance(12.583, 100.0).to_clock_dividers(None);

    // //配置传感器
    // let ch = Channel::Zero;
    // //为指定通道设置时钟分频器，这一参数控制采样速度，影响数据读取的频率。
    // //ldc.set_clock_dividers(ch, div).unwrap();
    // ldc.set_clock_dividers(ch, div).unwrap();

    // //设置转换稳定时间（settling time）为 40 个时钟周期。等待一段时间以确保信号稳定。
    // ldc.set_conv_settling_time(ch, 40).unwrap();

    // //设置参考计数转换间隔
    // ldc.set_ref_count_conv_interval(ch, 0x0546).unwrap();

    // //设置传感器驱动电流。这里的 0b01110 是一个二进制值，表示所需的电流设置
    // ldc.set_sensor_drive_current(ch, 0b01110).unwrap();

    // //配置 LDC1614 设备的多路复用器，使用单通道时with_auto_scan值为false
    // //设置去抖动滤波器带宽为 3.3 MHz
    // ldc.set_mux_config(
    //     MuxConfig::default()
    //         .with_auto_scan(false)
    //         .with_deglitch_filter_bandwidth(ldc1x1x::Deglitch::ThreePointThreeMHz),
    // )
    // .unwrap();
    // ldc.set_config(Config::default()).unwrap();
    // ldc.set_error_config(ErrorConfig::default().with_amplitude_high_error_to_data_register(true))
    //     .unwrap();

    // // timing ignored because polling with a cp2112 with no delays is slow enough already
    // // outputting just newline separated numbers so you can feed it into https://github.com/mogenson/ploot
    // loop {
    //     println!("{}", ldc.read_data_24bit(ch).unwrap(),);
    // }
}
