use super::{Error, Result};

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
#[derive(Debug)]
pub struct Handler {
    pub rx: Vec<(Consumer, broadcast::Receiver<super::Record>)>,
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
                        // wtr.write_record(&["timestamp", "data", "mark"]).unwrap(); // 我怀疑我们不用手动写一个表头
                        wtr
                    } else {
                        let reader = BufReader::new(&file);
                        let mut lines = reader.lines();
                        if let Some(Ok(first_line)) = lines.next() {
                            if first_line.trim() != "timestamp,data,mark" {
                                error!("错误的csv文件格式，将在文件末尾追加新内容");
                            }
                            let wtr = Writer::from_writer(file);
                            // wtr.write_record(&["timestamp", "data", "mark"]).unwrap(); // 我怀疑我们不用手动写一个表头
                            wtr
                        } else {
                            error!("无法读取csv文件");
                            return Err(Error::ConfigError);
                        }
                    };

                    #[derive(serde_derive::Serialize)]
                    struct CsvRecord {
                        /// 时间戳
                        timestamp: chrono::DateTime<chrono::Local>,
                        /// 数据
                        data: u32,
                        /// 通道
                        channel: u8,
                        /// 可选的标记
                        mark: Option<String>,
                    }

                    debug!("发布csv写入线程");
                    tokio::spawn(async move {
                        while let Ok(record) = rx.recv().await {
                            debug!("csv收到数据: {:?}", record);
                            wtr.serialize(CsvRecord {
                                mark: record
                                    .mark
                                    .clone()
                                    .map(|mark| format!("{}", mark).replace("\n", "\\n")),
                                timestamp: record.timestamp,
                                channel: match record.channel {
                                    Channel::Zero => 0,
                                    Channel::One => 1,
                                    Channel::Two => 2,
                                    Channel::Three => 3,
                                },
                                data: record.data,
                            })
                            .unwrap_or_else(|err| {
                                error!("写入csv文件失败, 数据为: {:?}, 错误: {:?}", record, err);
                            });
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
                                    use fred::prelude::TimeSeriesInterface;
                                    debug!("redis收到数据: {:?}", record);
                                    let key = match record.channel {
                                        Channel::Zero => "Channel0",
                                        Channel::One => "Channel1",
                                        Channel::Two => "Channel2",
                                        Channel::Three => "Channel3",
                                    };
                                    let key = if let Some(ref string) = record.mark {
                                        format!("{}:{}", key, string)
                                    } else {
                                        key.to_string()
                                    };
                                    client
                                        .ts_add::<usize, String, i64, RedisMap>(
                                            key,
                                            record.timestamp.timestamp(),
                                            record.data as f64,
                                            None,
                                            None,
                                            None,
                                            None,
                                            fred::types::RedisMap::new(),
                                        )
                                        .await
                                        .unwrap_or_else(|err| {
                                            error!(
                                                "写入redis失败, 数据为: {:?}, 错误: {:?}",
                                                record, err
                                            );
                                            0
                                        });
                                }
                                Err(e) => {
                                    error!("redis消费错误: {:?}", e);
                                }
                            }
                        }
                        // debug!("redis消费线程正在关闭");
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
        tx.send(crate::Record {
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
        .unwrap();
    }

    use std::time::Duration;
    tokio::time::sleep(Duration::from_secs(5)).await;
}
