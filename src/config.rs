use log::debug;

use crate::{Error, Result};

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Register {
    field: String,
    value: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    registers: Vec<Register>,
    channel: u8,
    count: usize,
    location: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Redis {
    url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Csv {
    path: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    csv: Option<Csv>,
    redis: Option<Redis>,
    tasks: Vec<Task>,
}

impl Config {
    pub fn read_config(path: &str) -> Result<Config> {
        use super::Error;
        use serde::de::Deserialize;
        use serde_yaml::Deserializer;
        use std::{fs::File, io::BufReader};

        let file = File::open(path).expect("无法打开文件");
        let reader = BufReader::new(file);
        let deserializer = Deserializer::from_reader(reader);
        Deserialize::deserialize(deserializer).map_err(|_| Error::ConfigError)
    }
}

use crate::handler;

impl TryFrom<Config> for (Vec<Task>, handler::Handler) {
    type Error = super::Error;
    fn try_from(value: Config) -> std::result::Result<Self, Self::Error> {
        use handler::Consumer;
        use std::{path::Path, sync::Arc};

        debug!("创建广播通道");
        let (tx, _) = tokio::sync::broadcast::channel(512);

        let mut consumers = vec![];
        if let Some(csv) = value.csv {
            debug!("发现csv配置");
            consumers.push((
                Consumer::Csv {
                    path: csv.path.map(|p| Arc::from(Path::new(&p))),
                },
                tx.subscribe(),
            ));
        }

        if let Some(redis) = value.redis {
            debug!("发现redis配置");
            consumers.push((
                Consumer::Redis {
                    url: redis.url.into(),
                },
                tx.subscribe(),
            ));
        }

        let handler = handler::Handler { rx: consumers };
        unimplemented!()
    }
}

// pub fn write_register_from_yml(ldc: Ldc, config: Task) {
//     use ldc1614::bitmap::LdcRegister;
//     for reg in config.registers {
//         match reg.field.as_str() {
//             "rcountx" => {
//                 let val = reg.value as u16;
//                 LdcRegister.rcountx.0.write(val);
//             }
//             "offsetx" => {
//                 let val = reg.value as u16;
//                 LdcRegister.rcountx.0.write(val);
//             }
//             "settlecountx" => {}
//             "clock_dividersx" => {}
//         }
//     }
// }
