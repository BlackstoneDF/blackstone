use base64::Engine;
use serde::{Deserialize, Serialize};

use super::block::Block;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VariableScope {
    #[default]
    Local,
    Unsaved,
    Saved,
}

impl VariableScope {
    pub fn to_json(&self) -> String {
        match self {
            VariableScope::Local => "local".to_string(),
            VariableScope::Unsaved => "unsaved".to_string(),
            VariableScope::Saved => "saved".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BracketType {
    Norm,
    Repeat,
}

impl BracketType {
    pub fn to_json(&self) -> String {
        match self {
            Self::Norm => "norm".to_string(),
            Self::Repeat => "repeat".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code, unused)]
pub enum BracketDirection {
    Open,
    Close,
}

#[allow(dead_code, unused)]
impl BracketDirection {
    pub fn to_json(&self) -> String {
        match self {
            Self::Open => "open".to_string(),
            Self::Close => "close".to_string(),
        }
    }
}

#[allow(dead_code, unused)]
pub fn process_block_vec(input: Vec<Block>) -> String {
    let mut out_str = String::new();
    out_str.push_str(r#"{"blocks":["#);
    for block in input {
        out_str.push_str(block.to_json().as_str());
        out_str.push(',');
    }
    out_str.pop();
    out_str.push_str(r#"]}"#);

    let mut data_as_bytes = out_str.as_bytes();
    let mut encoder = libflate::gzip::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut data_as_bytes, &mut encoder).unwrap();
    let compressed = encoder.finish().into_result().unwrap();

    base64::engine::general_purpose::STANDARD.encode(compressed)
}
