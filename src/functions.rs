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
    pub test_location: Vec<u8>,
}

pub fn read_config(file_path: &str) -> Result<Config> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    // 使用流式反序列化
    let mut deserializer = Deserializer::from_reader(reader);
    let config = Config::deserialize(deserializer).expect("Failed to parse config");
    Ok(config)
}
