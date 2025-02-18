use super::{Error, Result};
use crate::handler::a::Record;
use serde::Serialize;
use std::{
    fs::File,
    path::Path,
    sync::{self},
};
#[derive(Debug)]
pub enum Consumer {
    Csv { path: Option<sync::Arc<Path>> },
}

use log::{debug, error, info, warn};
use tokio::sync::broadcast;

// 定义 CsvRecordA 结构体
#[derive(Debug, Serialize)]
pub struct CsvRecordA {
    //pub timestamp: String,  // 时间戳
    pub mark: Option<String>, // 可选标记
    pub channel: u8,          // 通道
    pub data: u32,            // 数据
}

// 定义 CsvRecordB 结构体
#[derive(Debug, Serialize)]
pub struct CsvRecordB {
    // pub timestamp: String,  // 时间戳
    pub mark: Option<String>, // 可选标记
    pub channel: u8,          // 通道
    pub data: u32,            // 数据
    pub settlecount: u16,     // settlecount 寄存器值
    pub rcount: u16,          // rcount 寄存器值
}
#[derive(Debug)]
pub struct Handler {
    pub rx: Vec<(Consumer, broadcast::Receiver<a::Record>)>,
}

impl Handler {
    pub async fn submit(self) -> Result<()> {
        for (consumer, mut rx) in self.rx {
            match consumer {
                Consumer::Csv { path } => {
                    debug!("读取csv文件");
                    use chrono::Local;
                    let file = match path {
                        Some(path) => match path.extension() {
                            Some(ext) if ext == std::ffi::OsStr::new("csv") => {
                                if path.exists() {
                                    File::options()
                                        .append(true)
                                        .read(true)
                                        .open(&path)
                                        .map_err(|e| error!("无法打开csv文件 {:?} {:?}", path, e))
                                        .unwrap()
                                } else {
                                    warn!("配置文件中csv文件路径不存在，正在尝试创建该文件");
                                    File::create_new(&path)
                                        .map_err(|e| error!("无法创建文件 {:?} {:?}", path, e))
                                        .unwrap()
                                }
                            }
                            // 之后写解析配置文件的时候改一下
                            _ => return Err(Error::ConfigError),
                        },
                        None => {
                            let path = format!(
                                "data_{}.csv",
                                Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
                            );
                            let file = File::create_new(&path)
                                .map_err(|e| error!("无法创建文件 {:?} {:?}", path, e))
                                .unwrap();
                            info!(
                                "创建csv文件 {}{}",
                                std::env::current_dir()
                                    .map_err(|e| error!("无法获取到当前运行目录 {:?}", e))
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                                path
                            );
                            file
                        }
                    };

                    debug!("检查csv文件格式");
                    use csv::Writer;
                    use std::io::{BufRead, BufReader};
                    let mut wtr = if file.metadata().expect("无法读取文件元数据").len() == 0
                    {
                        let wtr = Writer::from_writer(file);
                        wtr
                    } else {
                        let reader = BufReader::new(file.try_clone().unwrap());
                        let mut lines = reader.lines();
                        if let Some(Ok(first_line)) = lines.next() {
                            if first_line.trim() != "timestamp,data,mark" {
                                error!("错误的csv文件格式，将在文件末尾追加新内容");
                            }
                            let wtr = Writer::from_writer(file);
                            wtr
                        } else {
                            error!("无法读取csv文件: {:?}", file);
                            return Err(Error::ConfigError);
                        }
                    };

                    debug!("发布csv写入线程");
                    tokio::spawn(async move {
                        while let Ok(record) = rx.recv().await {
                            debug!("csv收到数据: {:?}", record);
                            match record {
                                Record::Easy(record) => {
                                    wtr.serialize(record).unwrap_or_else(|err| {
                                        error!("写入csv文件失败, 错误: {:?}",err);
                                    });
                                }
                                Record::Reg(record) => {
                                    wtr.serialize(record).unwrap_or_else(|err| {
                                        error!("写入csv文件失败, 错误: {:?}",err);
                                    });
                                }
                                Record::All(record) => {
                                    wtr.serialize(record).unwrap_or_else(|err| {
                                        error!("写入csv文件失败, 错误: {:?}",err);
                                    });
                                }
                            }
                        }
                    });
                }
            }
        }
        Ok(())
    }
}

pub mod a {

    // 定义选择的记录类型
    #[derive(Debug, Clone)]
    pub enum Record {
        Easy(EasyRecord),
        Reg(RegRecord),
        All(AllRecord),
    }

    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    pub struct EasyRecord {
        pub position: f64,                          // 位置
        pub data: u32,                              // 数据
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct RegRecord {
        pub position: f64,                          // 位置
        pub data: u32,                              // 数据
        pub settlecount: u16,                       // settlecount寄存器值
        pub rcount: u16,                            // rcount寄存器值
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct AllRecord {
        pub position: f64,                          // 位置
        pub data: u32,                              // 数据
        pub channel: u8,                            // 通道
        pub settlecount: u16,                       // settlecount寄存器值
        pub rcount: u16,                            // rcount寄存器值
        pub date: chrono::DateTime<chrono::Local>,  // 日期
    }
}
