use super::{Error, Result};
use crate::handler::a::Record;
use fred::interfaces::TimeSeriesInterface;
use serde::Serialize;
use std::{
    fs::File,
    path::Path,
    sync::{self, Arc},
};
#[derive(Debug)]
pub enum Consumer {
    Redis { url: Arc<str> },
    Csv { path: Option<sync::Arc<Path>> },
}

use ldc1614::Channel;
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
                                // 处理 RecordA 类型的数据
                                Record::A(record_a) => {
                                    let csv_record_a = CsvRecordA {
                                        mark: record_a
                                            .mark
                                            .clone()
                                            .map(|mark| format!("{}", mark).replace("\n", "\\n")),
                                        //timestamp: record_a.timestamp.to_string(),
                                        channel: match record_a.channel {
                                            Channel::Zero => 0,
                                            Channel::One => 1,
                                            Channel::Two => 2,
                                            Channel::Three => 3,
                                        },
                                        data: record_a.data,
                                    };

                                    wtr.serialize(csv_record_a).unwrap_or_else(|err| {
                                        error!(
                                            "写入csv文件失败, 数据为: {:?}, 错误: {:?}",
                                            record_a, err
                                        );
                                    });
                                }

                                // 处理 RecordB 类型的数据
                                Record::B(record_b) => {
                                    let csv_record_b = CsvRecordB {
                                        mark: record_b
                                            .mark
                                            .clone()
                                            .map(|mark| format!("{}", mark).replace("\n", "\\n")),
                                        //timestamp: record_b.timestamp.to_string(),
                                        channel: match record_b.channel {
                                            Channel::Zero => 0,
                                            Channel::One => 1,
                                            Channel::Two => 2,
                                            Channel::Three => 3,
                                        },
                                        data: record_b.data,
                                        settlecount: record_b.settlecount,
                                        rcount: record_b.rcount,
                                    };

                                    wtr.serialize(csv_record_b).unwrap_or_else(|err| {
                                        error!(
                                            "写入csv文件失败, 数据为: {:?}, 错误: {:?}",
                                            record_b, err
                                        );
                                    });
                                }
                            }
                        }
                    });
                }
                Consumer::Redis { url } => {
                    debug!("初始化redis数据库");
                    use fred::{
                        interfaces::ClientLike,
                        types::{Builder, RedisConfig, RedisMap},
                    };

                    // 格式为redis://username:password@foo.com:6379/1
                    let config = RedisConfig::from_url(&url)?;
                    let client = Builder::from_config(config).build()?;
                    let _connection_task = client.init().await?;

                    // client.quit().await?; // 之后写信号捕捉的时候移过去
                    tokio::spawn(async move {
                        debug!("redis消费线程创建成功");
                        loop {
                            match rx.recv().await {
                                Ok(record) => {
                                    debug!("redis收到数据: {:?}", record);

                                    // 根据不同的 Record 类型 (Record::A 或 Record::B) 进行匹配
                                    match record {
                                        Record::A(record_a) => {
                                            // 处理 RecordA 类型的数据
                                            let key = match record_a.channel {
                                                ldc1614::Channel::Zero => "Channel0",
                                                ldc1614::Channel::One => "Channel1",
                                                ldc1614::Channel::Two => "Channel2",
                                                ldc1614::Channel::Three => "Channel3",
                                            };

                                            // 处理 mark，如果有则加入到 key 中
                                            let key = if let Some(ref string) = record_a.mark {
                                                format!("{}:{}", key, string)
                                            } else {
                                                key.to_string()
                                            };

                                            // Redis 写入操作，可以使用 record_a 的数据
                                            client
                                                .ts_add::<usize, String, i64, RedisMap>(
                                                    key.clone(),
                                                    0, // 使用 0 或者替换为实际的 timestamp
                                                    record_a.data as f64,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    RedisMap::new(),
                                                )
                                                .await
                                                .unwrap_or_else(|err| {
                                                    error!(
                                                        "写入redis失败, 数据为: {:?}, 错误: {:?}",
                                                        record_a, err
                                                    );
                                                    0 // 返回默认的 usize 值
                                                });
                                        }
                                        Record::B(record_b) => {
                                            // 处理 RecordB 类型的数据
                                            let key = match record_b.channel {
                                                ldc1614::Channel::Zero => "Channel0",
                                                ldc1614::Channel::One => "Channel1",
                                                ldc1614::Channel::Two => "Channel2",
                                                ldc1614::Channel::Three => "Channel3",
                                            };

                                            // 处理 mark，如果有则加入到 key 中
                                            let key = if let Some(ref string) = record_b.mark {
                                                format!("{}:{}", key, string)
                                            } else {
                                                key.to_string()
                                            };

                                            // Redis 写入操作，可以使用 record_b 的数据
                                            client
                                                .ts_add::<usize, String, i64, RedisMap>(
                                                    key.clone(),
                                                    0, // 使用 0 或者替换为实际的 timestamp
                                                    record_b.data as f64,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    RedisMap::new(),
                                                )
                                                .await
                                                .unwrap_or_else(|err| {
                                                    error!(
                                                        "写入redis失败, 数据为: {:?}, 错误: {:?}",
                                                        record_b, err
                                                    );
                                                    0 // 返回默认的 usize 值
                                                });
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("redis消费错误: {:?}", e);
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

#[tokio::test]
async fn test_redis() {
    use env_logger::Builder;
    use log::debug;
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let (tx, rx) = tokio::sync::broadcast::channel(512);

    //redis://username:password@foo.com:6379/1
    let handler = Handler {
        rx: vec![(
            Consumer::Redis {
                //这是开发容器的ip
                url: "redis://:mypassword@172.19.0.2:6379".into(),
            },
            rx,
        )],
    };

    handler.submit().await.unwrap();

    use chrono::Local;
    for _ in 0..60 {
        debug!("数据发射");
        /*  tx.send(crate::Record {
            timestamp: Local::now(),
            data: 111,
            channel: Channel::Zero,
            mark: Some("test".into()),
        })
        .unwrap();
        tx.send(crate::Record {
            timestamp: Local::now(),
            data: 111,
            channel: Channel::Zero,
            mark: Some("test:but will not be selected".into()),
        })
        .unwrap(); */
        // 创建一个示例 Record，可以是 RecordA 或 RecordB
        let record = Record::A(a::RecordA {
            //timestamp: Local::now(),
            data: 111,
            channel: Channel::Zero,
            mark: Some("test".into()),
        });

        // 匹配 Record 类型，分别处理 RecordA 和 RecordB
        match record {
            // 处理 RecordA 类型
            Record::A(record_a) => {
                tx.send(Record::A(a::RecordA {
                    //timestamp: record_a.timestamp,
                    data: 111,
                    channel: Channel::Zero,
                    mark: Some("test".into()), // 复制 mark（如果是 Option 类型）
                }))
                .unwrap(); // 你可以根据需要修改 unwrap()，比如改成更优雅的错误处理
            }

            // 处理 RecordB 类型
            Record::B(record_b) => {
                tx.send(Record::B(a::RecordB {
                    //timestamp: record_b.timestamp,
                    data: 111,
                    channel: record_b.channel,
                    mark: Some("test".into()), // 复制 mark
                    settlecount: record_b.settlecount,
                    rcount: record_b.rcount,
                }))
                .unwrap(); // 你可以根据需要修改 unwrap()，比如改成更优雅的错误处理
            }
        }
    }

    use std::time::Duration;
    tokio::time::sleep(Duration::from_secs(5)).await;
}

pub mod a {

    // 定义选择的记录类型
    #[derive(Debug, Clone)]
    pub enum Record {
        A(RecordA), // 对应 RecordA
        B(RecordB), // 对应 RecordB
    }

    // RecordA 和 RecordB 的结构体
    #[derive(Debug, Clone)]
    pub struct RecordA {
        pub data: u32,                         // 数据
        pub channel: ldc1614::Channel,         // 通道
        pub mark: Option<std::sync::Arc<str>>, // 可选标记
    }

    #[derive(Debug, Clone)]
    pub struct RecordB {
        pub data: u32,                         // 数据
        pub channel: ldc1614::Channel,         // 通道
        pub mark: Option<std::sync::Arc<str>>, // 可选标记
        pub settlecount: u16,                  // settlecount寄存器值
        pub rcount: u16,                       // rcount寄存器值
    }

    impl Record {
        // 新的构造函数，分别为 RecordA 和 RecordB 提供专门的构造函数
        pub fn new_a(
            data: u32,
            channel: ldc1614::Channel,
            mark: Option<std::sync::Arc<str>>,
        ) -> Self {
            Record::A(RecordA {
                data,
                channel,
                mark,
            })
        }

        pub fn new_b(
            data: u32,
            channel: ldc1614::Channel,
            mark: Option<std::sync::Arc<str>>,
            settlecount: u16,
            rcount: u16,
        ) -> Self {
            Record::B(RecordB {
                data,
                channel,
                mark,
                settlecount,
                rcount,
            })
        }
    }

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)] // 添加 Deserialize 和 Serialize 派生
    pub enum RecordType {
        A, // 对应 RecordA
        B, // 对应 RecordB
    }
}
