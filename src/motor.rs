use super::Result;

use rppal::gpio;
pub struct Motor {
    pwm: gpio::OutputPin,
    dir: gpio::OutputPin,
}

impl Motor {
    pub fn new(pwm: gpio::OutputPin, dir: gpio::OutputPin) -> Self {
        Motor { pwm, dir }
    }

    /// GPIO模拟PWM输出
    /// 首先说明电机的连接方式是共阴极连接，
    /// ENA-、DIR-、PUL-接控制器的地，
    /// ENA+接使能信号，DIR+接方向信号物理口32，PUL+接脉冲信号物理口40
    /// * distance 电机运动距离（正数为正向移动）
    async fn moving(&mut self, distance: f64) -> Result<()> {
        use tokio::time::{self, Duration};
        // 电机运动速度 (mm/s)= 脉冲频率 * 丝杆导程 * 电机旋转步长 / (编码器细分度 * 360°)
        // > 滑轨每转的距离是1.0mm，步进电机在没有细分的情况下，电机每步进一次时的角度为1.8°
        const SPEED: f64 = 1.5625;
        // 启动 PWM 输出
        self.pwm.set_pwm_frequency(5000.0, 0.5)?;
        //设置方向引脚
        match distance.is_sign_negative() {
            true => {
                self.dir.set_low();
            }
            false => {
                self.dir.set_high();
            }
        }
        // 异步等待滑轨前进指定距离
        time::sleep(Duration::from_secs_f64(distance.abs() / SPEED)).await;
        // 停止 PWM 输出
        self.pwm.clear_pwm()?;
        Ok(())
    }
}
