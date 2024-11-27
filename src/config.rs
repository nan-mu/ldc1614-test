use std::fmt::Debug;

use crate::Result;

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Register {
    pub field: String,
    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    pub registers: Option<Vec<Register>>,
    channel: u8,
    pub count: usize,
    location: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Csv {
    pub path: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub csv: Option<Csv>,
    pub redis: Option<Redis>,
    pub tasks: Vec<Task>,
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
        let config = Deserialize::deserialize(deserializer)
            as std::result::Result<Config, serde_yaml::Error>;
        match config {
            Ok(mut config) => {
                config.tasks.iter_mut().for_each(|task| {
                    if let Some(_) = &task.registers {
                        todo!("检查是否所有寄存器设置选项都支持");
                    }
                });
                Ok(config)
            }
            Err(e) => {
                log::error!("{:?}", e);
                Err(Error::ConfigError)
            }
        }
    }
}

#[tokio::test]
async fn test_yml() {
    use std::path::Path;
    if !Path::new("/workspaces/tasks.yml").exists() {
        panic!("配置文件不存在");
    }

    match Config::read_config("/workspaces/tasks.yml") {
        Ok(config) => {
            println!("{:?}", config);
            assert!(config.tasks.len() > 0, "任务列表不能为空");
        }
        Err(e) => panic!("读取配置文件失败: {}", e),
    }
}

impl Task {
    pub fn channel(&self) -> ldc1614::Channel {
        match self.channel {
            0 => ldc1614::Channel::Zero,
            1 => ldc1614::Channel::One,
            2 => ldc1614::Channel::Two,
            3 => ldc1614::Channel::Three,
            _ => panic!("Invalid channel number"),
        }
    }

    pub fn position(&self) -> Vec<f64> {
        let (start, step, end): (f64, f64, f64) = matlab_type_range(&self.location);
        let mut result = Vec::new();
        let mut current = start;
        while current <= end {
            result.push(current);
            current += step;
        }
        result
    }
}

pub fn matlab_type_range<T: std::str::FromStr<Err = E> + Copy, E: std::fmt::Debug>(
    string: &str,
) -> (T, T, T) {
    let parts: Vec<&str> = string.split(':').collect();
    let start: T = parts[0].parse().unwrap();

    match parts.len() {
        1 => (start, "1".parse().unwrap(), start),
        2 => (start, "1".parse().unwrap(), parts[1].parse().unwrap()),
        3 => (start, parts[1].parse().unwrap(), parts[2].parse().unwrap()),
        _ => {
            log::error!("无法转换 \"{string}\" 为matlab的范围");
            panic!()
        }
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
