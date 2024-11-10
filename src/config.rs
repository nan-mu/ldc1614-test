use crate::Result;

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Register {
    pub field: String,
    pub value: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    pub registers: Vec<Register>,
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
        Deserialize::deserialize(deserializer).map_err(|_| Error::ConfigError)
    }
}

impl Task {
    pub fn registers(&self) -> Vec<Register> {
        self.registers.clone()
    }
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
        let parts: Vec<&str> = self.location.split(':').collect();
        let start: f64;
        let step: f64;
        let end: f64;

        match parts.len() {
            1 => {
                start = parts[0].parse().unwrap();
                step = 1.0;
                end = start;
            }
            2 => {
                start = parts[0].parse().unwrap();
                step = 1.0;
                end = parts[1].parse().unwrap();
            }
            3 => {
                start = parts[0].parse().unwrap();
                step = parts[1].parse().unwrap();
                end = parts[2].parse().unwrap();
            }
            _ => panic!("Invalid location format"),
        }

        let mut result = Vec::new();
        let mut current = start;
        while current <= end {
            result.push(current);
            current += step;
        }
        result
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
