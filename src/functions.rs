use serde::Deserialize;
use serde_yaml::Deserializer;
use std::fs::File;
use std::io::{BufReader, Result};

#[derive(Debug, Deserialize)]
pub struct Register {
    field: String,
    value: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub registers: Vec<Register>,
    pub test_count: u32,
    pub test_location: Vec<f64>,
}

pub fn read_config(file_path: &str) -> Result<Config> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    // 使用流式反序列化
    let deserializer = Deserializer::from_reader(reader);
    let config = Config::deserialize(deserializer).expect("Failed to parse config");
    Ok(config)
}

pub fn write_register_from_yml(ldc: Ldc, config: Config) {
    use ldc1614::bitmap::LdcRegister;
    for reg in config.registers {
        match reg.field.as_str() {
            "rcountx" => {
                let val = reg.value as u16;
                LdcRegister.rcountx.0.write(val);
            }
            "offsetx" => {
                let val = reg.value as u16;
                LdcRegister.rcountx.0.write(val);
            }
            "settlecountx" => {}
            "clock_dividersx" => {}
        }
    }
}
