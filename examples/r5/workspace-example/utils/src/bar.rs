use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddResponse {
    pub output: u64,
}

pub fn add(left: u64, right: u64) -> AddResponse {
    AddResponse {
        output: left + right,
    }
}
