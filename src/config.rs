use std::{collections::HashMap, fmt::Debug, ops::Add};

use crate::Result;

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    pub registers: Option<HashMap<String, String>>,
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
        let (start, step, end): (f64, f64, f64) = matlab_type_range(&self.location);
        let mut result = Vec::new();
        let mut current = start;
        while current < end {
            result.push(current);
            current += step;
        }
        result
    }
}

pub fn matlab_type_range<
    T: std::str::FromStr<Err = E> + Copy + Add<Output = T>,
    E: std::fmt::Debug,
>(
    string: &str,
) -> (T, T, T) {
    let parts: Vec<&str> = string.split(':').collect();
    let start: T = parts[0].parse().unwrap();

    match parts.len() {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yml() {
        use std::path::Path;
        if !Path::new("/workspaces/tasks/task-example.yml").exists() {
            panic!("配置文件不存在");
        }

        match Config::read_config("/workspaces/tasks/task-example.yml") {
            Ok(config) => {
                println!("{:?}", config);
                assert!(config.tasks.len() > 0, "任务列表不能为空");
            }
            Err(e) => panic!("读取配置文件失败: {}", e),
        }
    }

    #[test]
    fn test_matlab_type_range_single_value() {
        let input = "10";
        let expected_output = (10, 1, 11);
        assert_eq!(matlab_type_range(input), expected_output);
        let input = "10:20";
        let expected_output = (10, 1, 21);
        assert_eq!(matlab_type_range(input), expected_output);
        let input = "10:2:20";
        let expected_output = (10, 2, 21);
        assert_eq!(matlab_type_range(input), expected_output);
    }

    #[test]
    #[should_panic]
    fn test_matlab_type_range_invalid_format() {
        let input = "10:20:30:40";
        matlab_type_range::<i32, _>(input);
    }
}
