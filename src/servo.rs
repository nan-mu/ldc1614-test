use super::Result;
use log::debug;
use rppal::uart;
use tokio::time::{self, Duration}; // 确保 Result 正确导入

pub struct Servo {
    uart: uart::Uart,
    position: f64,
}

impl Servo {
    pub fn new(uart: uart::Uart) -> Self {
        Servo {
            uart,
            position: 0.0,
        }
    }

    pub async fn goto(&mut self, target: f64) -> Result<()> {
        debug!("舵机从 {}度 移动到 {}度", self.position, target);

        const SCALE: f64 = 0.09; // 范围 500-2500 对应的舵机角度就是 0-180 度，一个单位是 0.09 度
        const STOP: &'static str = "#000PDPT!"; // 定义暂停指令

        // 计算并格式化发送的数据
        let send_data = format!(
            "#000P{:04}T0100!",
            ((target / SCALE) + 500.0).round() as u32
        );

        // 发送数据
        self.uart.write(send_data.as_bytes())?;
        println!("Data sent: {:?}", send_data);

        // 假设 time 是一个延迟时间，你可以根据需要调整这个值
        let delay_time_ms: u64 = 100; // 延时100ms
        time::sleep(Duration::from_millis(delay_time_ms as u64)).await;

        // 发送停止指令
        self.uart.write(STOP.as_bytes())?;
        println!("Stop: {:?}", STOP);

        // 更新舵机位置
        self.position = target;

        Ok(())
    }

    pub fn set_position(&mut self, position: f64) {
        self.position = position; // 更新舵机的角度
    }
}

//use rppal::uart::{Settings, Uart};
// fn uart_init() -> Result<(), Box<dyn std::error::Error>> {
//     // 创建串口设置
//     let settings = Settings {
//         baudrate: 115200,
//         bytesize: uart::ByteSize::Eight,
//         parity: uart::Parity::None,
//         stopbits: uart::StopBits::One,
//     };

//     // 打开串口（比如使用 /dev/serial0）
//     let mut uart = Uart::new("/dev/serial0", &settings)?;

//     Ok(())
// }

#[tokio::test]
async fn test_goto() {
    let gpio = gpio::Gpio::new().unwrap();
    let pwm_pin = gpio.get(21).unwrap().into_output();
    let dir_pin = gpio.get(12).unwrap().into_output();
    let mut motor = Servo::new(pwm_pin, dir_pin);

    motor.goto(15.0).await.unwrap();
    assert_eq!(motor.position, 15.0);

    motor.goto(0.0).await.unwrap();
    assert_eq!(motor.position, 0.0);
}
