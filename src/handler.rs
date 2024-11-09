use super::{Error, Result};

use std::{
    fs::File,
    path::{self, Path},
    sync,
};
#[derive(Debug)]
enum Consumer {
    Redis,
    Csv { address: Option<sync::Arc<Path>> },
}

use log::{debug, error, info, warn};
use tokio::sync::broadcast;
#[derive(Debug)]
struct Handler {
    rx: Vec<(Consumer, broadcast::Receiver<super::Record>)>,
}

impl Handler {
    async fn submit(self) -> Result<()> {
        for (consumer, mut rx) in self.rx {
            match consumer {
                Consumer::Csv { address } => {
                    debug!("读取csv文件");
                    use chrono::Local;
                    let file = match address {
                        Some(path) => match path.extension() {
                            Some(ext) if ext == std::ffi::OsStr::new("csv") => {
                                if path.exists() {
                                    File::open(&path).expect(&format!("无法打开csv文件 {:?}", path))
                                } else {
                                    warn!("配置文件中csv文件路径不存在，正在尝试创建该文件");
                                    File::create_new(&path)
                                        .expect(&format!("无法创建文件 {:?}", path))
                                }
                            }
                            // 之后写解析配置文件的时候改一下
                            _ => return Err(Error::ConfigErr),
                        },
                        None => {
                            let filename = format!(
                                "data_{}.csv",
                                Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
                            );
                            let file = File::create_new(&filename)
                                .expect(&format!("无法创建文件 {:?}", filename));
                            info!(
                                "创建csv文件 {}{}",
                                std::env::current_dir()
                                    .expect("无法获取到当前运行目录")
                                    .to_str()
                                    .unwrap(),
                                filename
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
                            return Err(Error::ConfigErr);
                        }
                    };

                    #[derive(serde_derive::Serialize)]
                    struct CsvRecord<'a> {
                        /// 时间戳
                        timestamp: chrono::DateTime<chrono::Local>,
                        /// 数据
                        data: u32,
                        /// 可选的标记
                        mark: Option<&'a str>,
                    }

                    debug!("发布csv写入线程");
                    tokio::spawn(async move {
                        while let Ok(record) = rx.recv().await {
                            debug!("csv收到数据: {:?}", record);
                            wtr.serialize(CsvRecord {
                                mark: record.mark.as_deref(),
                                timestamp: record.timestamp,
                                data: record.data,
                            })
                            .unwrap_or_else(|err| {
                                error!("写入csv文件失败, 数据为: {:?}, 错误: {:?}", record, err);
                            });
                        }
                    });
                }
                Consumer::Redis => {
                    tokio::spawn(async move {
                        while let Ok(record) = rx.recv().await {
                            println!("Received record: {:?}", record);
                        }
                    });
                }
            }
        }

        Ok(())
    }
}
