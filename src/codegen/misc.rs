use base64::Engine;
use serde::{Deserialize, Serialize};

use super::block::Block;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum VariableScope {
    Local,
    Unsaved,
    Saved,
}

impl VariableScope {
    pub fn to_json(self) -> String {
        match self {
            VariableScope::Local => format!("local"),
            VariableScope::Unsaved => format!("unsaved"),
            VariableScope::Saved => format!("saved"),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum BracketType {
    Norm,
    Repeat,
}

impl BracketType {
    pub fn to_json(self) -> String {
        match self {
            Self::Norm => format!("norm"),
            Self::Repeat => format!("repeat"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BracketDirection {
    Open,
    Close,
}

impl BracketDirection {
    pub fn to_json(self) -> String {
        match self {
            Self::Open => format!("open"),
            Self::Close => format!("close"),
        }
    }
}

pub fn process_block_vec(input: Vec<Block>) -> String {
    let mut out_str = String::new();
    out_str.push_str(r#"{"blocks":["#);
    for block in input {
        out_str.push_str(block.to_json().as_str());
        out_str.push_str(",");
    }
    out_str.pop();
    out_str.push_str(r#"]}"#);

    let mut data_as_bytes = out_str.as_bytes();
    let mut encoder = libflate::gzip::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut data_as_bytes, &mut encoder).unwrap();
    let compressed = encoder.finish().into_result().unwrap();
    let b64 = base64::engine::general_purpose::STANDARD.encode(&compressed);

    b64
}
