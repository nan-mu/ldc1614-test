use super::Result;

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
    url: Option<String>,
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

pub fn read_config(file_path: &str) -> Result<Config> {
    use super::Error;
    use serde::de::Deserialize;
    use serde_yaml::Deserializer;
    use std::{fs::File, io::BufReader};

    let file = File::open(file_path).expect("无法打开文件");
    let reader = BufReader::new(file);
    let deserializer = Deserializer::from_reader(reader);
    Deserialize::deserialize(deserializer).map_err(|_| Error::ConfigError)
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
