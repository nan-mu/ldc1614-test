//! 包含从ldc1614::Ldc得到对应通道代码的结构。最后的形式是从一个tokio的广播结构发送数据

use super::{Error, Result};

use chrono::Local;
use tokio::sync::{self, broadcast};

struct Channel<const ADDR: u8> {
    ldc: std::sync::Arc<sync::Mutex<ldc1614::Ldc<ADDR>>>,
    i2c: std::sync::Arc<sync::Mutex<rppal::i2c::I2c>>,
    channel: ldc1614::Channel,
    rx: broadcast::Sender<super::Record>,
}

impl<const ADDR: u8> Channel<ADDR> {
    async fn submit(&self, mark: Option<std::sync::Arc<str>>) -> Result<usize> {
        let (ldc, mut i2c) = tokio::join!(self.ldc.lock(), self.i2c.lock());
        Ok(self.rx.send(crate::Record {
            timestamp: Local::now(),
            data: ldc
                .read_data(&mut *i2c, self.channel)
                .map_err(|e| Error::Ldc1614(e))?,
            channel: self.channel,
            mark,
        })?)
    }
}
