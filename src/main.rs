// i2c_ds3231.rs - Sets and retrieves the time on a Maxim Integrated DS3231
// RTC using I2C.

use ldc1x1x as ldc;
use rppal::i2c::I2c;
//ADDR接地时ldc1614的地址
const ADDR_LDC1614: u8 = 0x2A;

fn automatic_calibration() {
    let mut i2c = I2c::new().unwrap(); //创建i2c实例
    let mut ldc = ldc::Ldc::new(&mut i2c, ADDR_LDC1614);

    // 1. 创建默认配置并将设备置于 SLEEP 模式
    let config = Config(0);
    let new_config_sleep = config.with_active_chan(Channel::Zero).with_sleep_mode(true);

    // 应用配置到 LDC 设备
    ldc.set_config(new_config_sleep).unwrap();

    // 2. 为通道编写所需的 SETTLECOUNT 和 RCOUNT 值
    let ch = Channel::Zero;
    ldc.set_conv_settling_time(ch, 40).unwrap();
    ldc.set_ref_count_conv_interval(ch, 0x0546).unwrap();

    // 3. 设置自动校准
    let new_config_auto_cal = config
        .with_active_chan(Channel::Zero)
        .with_sleep_mode(true)
        .with_automatic_sensor_amplitude_correction(false);

    // 应用新的配置
    ldc.set_config(new_config_auto_cal).unwrap();

    // 4. 使设备退出 SLEEP 模式
    let new_config_wakeup = config
        .with_active_chan(Channel::Zero)
        .with_sleep_mode(false);

    // 应用配置以退出 SLEEP 模式
    ldc.set_config(new_config_wakeup).unwrap();

    // 5. 允许设备至少执行一次测量

    // 6. 读取 DRIVE_CURRENTx 寄存器中的 INIT_DRIVEx 字段
    let drive_current = ldc.read_drive_current(ch).unwrap();
    let init_drive_value = (drive_current >> 6) & 0x1F; // 取出位 10:6

    // 7. 将保存的值写入 IDRIVEx 位字段
    ldc.write_idrive(ch, init_drive_value).unwrap();

    // 8. 设置固定电流驱动的 RP_OVERRIDE_EN 为 b1
    let new_config_fixed_current = config
        .with_active_chan(Channel::Zero)
        .with_rp_override_en(true);

    // 应用新的配置
    ldc.set_config(new_config_fixed_current).unwrap();
}

fn main() {
    let mut i2c = I2c::new().unwrap();

    //创建实例，初始化LDC设备
    let mut ldc = ldc::Ldc::new(&mut i2c, ADDR_LDC1614);

    ldc.reset().unwrap();

    //计算分频，这里需要理论计算
    let div = ldc::Fsensor::from_inductance_capacitance(12.583, 100.0).to_clock_dividers(None);

    //配置传感器
    for ch in [ldc::Channel::Zero] {
        //为指定通道设置时钟分频器，这一参数控制采样速度，影响数据读取的频率。
        //ldc.set_clock_dividers(ch, div).unwrap();
        ldc.set_clock_dividers(ch, div).unwrap();

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
            .with_deglitch_filter_bandwidth(ldc::Deglitch::ThreePointThreeMHz),
    )
    .unwrap();
    ldc.set_config(ldc::Config::default()).unwrap();
    ldc.set_error_config(
        ldc::ErrorConfig::default().with_amplitude_high_error_to_data_register(true),
    )
    .unwrap();

    // timing ignored because polling with a cp2112 with no delays is slow enough already
    // outputting just newline separated numbers so you can feed it into https://github.com/mogenson/ploot
    loop {
        println!("{}", ldc.read_data_24bit(ldc::Channel::Zero).unwrap(),);
    }
}
