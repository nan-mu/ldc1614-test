use super::Result;

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

    /// GPIO模拟PWM输出
    /// 首先说明电机的连接方式是共阴极连接，
    /// ENA-、DIR-、PUL-接控制器的地，
    /// ENA+接使能信号，DIR+接方向信号物理口32，PUL+接脉冲信号物理口40
    /// * distance 电机运动距离（正数为正向移动）
    async fn moving(&mut self, target: f64) -> Result<()> {
        use tokio::time::{self, Duration};
        let scale=0.09;//范围 500-2500 对应的舵机角度就是 0-180 度,一个单位是0.09度
        let mut pwm=target/scale+500;//pwm与所转角度的公式,
        let time=0100;
        
        let mut zhiling=00000//ID号+pwm+时间的指令拼接
        let stop="#000PDPT!";//定义暂停指令，是一个字符串
        // 发送数据
        
        uart.write(zhiling)?;

        println!("Data sent: {:?}", zhiling);

      
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


extern crate serial;
extern crate tokio; // 用于异步操作

use serial::prelude::*;
use std::io::{self, Write};
use std::time::Duration;
use tokio; // 引入异步运行时

pub struct Servo {
    uart: uart::Uart,
    angle: f64,
}

impl Servo {
    pub fn new(uart: uart::Uart) -> Self {
        Servo {
            uart,
            angle: 0.0,
        }
    }


#[tokio::main] // 标记入口函数为异步函数
async fn main() -> io::Result<()> {
    // 配置串口参数
    let port_name = "/dev/ttyUSB0"; // 根据你的设备可能需要更改串口路径
    let baud_rate = serial::BaudRate::Baud9600; // 设置波特率

    // 打开串口
    let port = serial::open(port_name).expect("无法打开串口");
    
    // 配置串口
    let mut port = serial::SerialPort::from(port);
    port.reconfigure(&|settings| {
        settings baud_rate(baud_rate)
            .timeout(Duration::from_millis(10)) // 设置超时
            .parity(serial::Parity::None) // 无奇偶校验位
            .stop_bits(serial::StopBits::One) // 一个停止位
            .data_bits(serial::DataBits::Eight) // 八个数据位
            .flow_control(serial::FlowControl::None) // 无流控
    })?;

    // 异步地发送命令到舵机
    tokio::task::spawn_blocking(move || -> io::Result<()> {
        let command = "your_command_here\n"; // 替换为实际控制舵机的命令
        port.write(command.as_bytes())?;
        port.flush()?;
        Ok(())
    })
    .await
    .expect("舵机控制任务失败")?;

    println!("命令已发送到舵机");

    Ok(())
}
