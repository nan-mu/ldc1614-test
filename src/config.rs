use std::{collections::HashMap, fmt::Debug};
use crate::Result;

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    pub registers: Option<HashMap<String, String>>,
    channel: u8,
    pub count: usize,
    location: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Csv {
    pub path: Option<String>,
    pub record_type: Option<RecordType>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub csv: Option<Csv>,
    pub tasks: Vec<Task>,
    pub drive_mode: Option<DriveMode>,
    pub display_pb: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DriveMode {
    Reciprocating,
    Direct,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordType {
    Easy,
    WithRegister,
    All,
}

use once_cell::sync::OnceCell;
pub static RECODE_TYPE: OnceCell<RecordType> = OnceCell::new();

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
                    if let Some(registers) = &mut task.registers {
                        use log::debug;
                        if !registers.contains_key("offset") {
                            debug!("自动补充offset为0x0");
                            registers.insert("offset".to_string(), "0".to_string());
                        }
                        if !registers.contains_key("fin_divider") {
                            debug!("自动补充fin_divider为0x1");
                            registers.insert("fin_divider".to_string(), "1".to_string());
                        }
                        if !registers.contains_key("fref_divider") {
                            debug!("自动补充fref_divider为0x2");
                            registers.insert("fref_divider".to_string(), "2".to_string());
                        }
                        if !registers.contains_key("deglitch") {
                            debug!("自动补充deglitch为0x1");
                            registers.insert("deglitch".to_string(), "1".to_string());
                        }
                        if !registers.contains_key("LC_sensor_drive_current") {
                            debug!("自动补充LC_sensor_drive_current为0x1f");
                            registers
                                .insert("LC_sensor_drive_current".to_string(), "31".to_string());
                        }
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
        matlab_type_range(&self.location)
    }
}

pub fn matlab_type_range<
    T: std::str::FromStr<Err = E> + Copy + std::ops::Add<Output = T> + std::cmp::PartialOrd,
    E: std::fmt::Debug,
>(
    string: &str,
) -> Vec<T> {
    if string.contains(',') {
        string
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect()
    } else {
        let parts: Vec<&str> = string.split(':').collect();
        let start: T = parts[0].parse().unwrap();

        let (start, step, end) = match parts.len() {
            1 => (start, "1".parse().unwrap(), start + "1".parse().unwrap()),
            2 => (
                start,
                "1".parse().unwrap(),
                parts[1].parse::<T>().unwrap() + "1".parse().unwrap(),
            ),
            3 => (
                start,
                parts[1].parse().unwrap(),
                parts[2].parse::<T>().unwrap() + "1".parse().unwrap(),
            ),
            _ => {
                log::error!("无法转换 \"{string}\" 为matlab的范围");
                panic!()
            }
        };
        let mut result = Vec::new();
        let mut current = start;
        while current < end {
            result.push(current);
            current = current + step;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yml() {
        use std::path::Path;
        if !Path::new("./tasks/task-5k.yml").exists() {
            panic!("配置文件不存在");
        }

        match Config::read_config("./tasks/task-5k.yml") {
            Ok(config) => {
                println!("{:?}", config);
                assert!(config.tasks.len() > 0, "任务列表不能为空");
            }
            Err(e) => panic!("读取配置文件失败: {}", e),
        }
    }

    #[test]
    fn test_matlab_type_range_single_value() {
        let input = "1160";
        let expected_output: Vec<f64> = (1160..1161).map(|x| x as f64).collect();
        assert_eq!(matlab_type_range(input) as Vec<f64>, expected_output);
        let input = "0:10";
        let expected_output: Vec<f64> = (0..11).map(|x| x as f64).collect();
        assert_eq!(matlab_type_range(input) as Vec<f64>, expected_output);
        let input = "10:2:20";
        let expected_output: Vec<f64> = (10..21).step_by(2).map(|x| x as f64).collect();
        assert_eq!(matlab_type_range(input) as Vec<f64>, expected_output);
        let input = "1,2,3,4";
        let expected_output: Vec<f64> = vec![1., 2., 3., 4.];
        assert_eq!(matlab_type_range(input) as Vec<f64>, expected_output);
    }

    #[test]
    #[should_panic]
    fn test_matlab_type_range_invalid_format() {
        let input = "10:20:30:40";
        matlab_type_range::<i32, _>(input);
    }
}
