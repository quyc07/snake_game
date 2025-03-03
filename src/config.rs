#![allow(dead_code)] // Remove this once you start using the code

use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

const DATA_PATH: &str = ".data/data.json";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataConfig {
    #[serde(default)]
    pub scores: Vec<u32>,
}

impl DataConfig {
    pub(crate) fn write_score(&mut self, score: u32) {
        let mut scores = self.scores.clone();
        scores.push(score);
        scores.sort_by(|x, x1| x1.cmp(x));
        let data = json5::to_string(&DataConfig {
            scores: scores[..min(scores.len(), 10)].to_vec(),
        })
        .unwrap();
        fs::write(DATA_PATH, data).unwrap();
    }
}

impl DataConfig {
    pub fn new() -> Result<Self, Error> {
        let data_config = json5::from_str(
            String::from_utf8(match fs::read(DATA_PATH) {
                Ok(data) => data,
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    let buf = PathBuf::from(DATA_PATH);
                    fs::create_dir_all(buf.parent().unwrap())?;
                    return Ok(DataConfig { scores: vec![] });
                }
                Err(e) => return Err(e),
            })
            .unwrap()
            .as_str(),
        )
        .unwrap();
        Ok(data_config)
    }
}
