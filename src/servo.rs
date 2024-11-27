use super::Result;
//舵机配置工作模式：#000PMOD3!，

use log::debug;
use rppal::uart;
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
        self.moving(target /*- self.position*/).await?;//转到target的位置
        self.position = target;
        Ok(())
    }

    async fn moving(&mut self, target: f64) -> Result<()> {
        use tokio::time::{self, Duration};
        let scale=0.09;//范围 500-2500 对应的舵机角度就是 0-180 度,一个单位是0.09度
        let ID="000P";
        let mut pwm=target/scale+500;//pwm与所转角度的公式,
        let pwm_str = format!("{:04}", pwm as u32);
        
        let mut send_data=format!("#000P{}T0100!", pwm_str);//ID号+pwm+时间的指令拼接
        let stop="#000PDPT!";//定义暂停指令，是一个字符串
        // 发送数据
        
        uart.write(send_data)?;

        println!("Data sent: {:?}",send_data);

      
        // 异步等待滑轨前进指定距离
        time::sleep(Duration::from_secs_f64(time / 1000)).await;
        // 停止 PWM 输出
        uart.write(stop)?;

        println!("stop: {:?}", stop);
        Ok(())
    }

    pub fn set_position(&mut self, position: f64) {
        self.position = position;//初始的角度
    }
}

use rppal::uart::{Uart, Settings};

fn uart_init() -> Result<(), Box<dyn std::error::Error>> {
    // 创建串口设置
    let settings = Settings {
        baudrate: 115200,
        bytesize: uart::ByteSize::Eight,
        parity: uart::Parity::None,
        stopbits: uart::StopBits::One,
    };

    // 打开串口（比如使用 /dev/serial0）
    let mut uart = Uart::new("/dev/serial0", &settings)?;

  

    Ok(())
}




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


