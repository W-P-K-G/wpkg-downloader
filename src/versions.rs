use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Versions{
    pub version: u32,
    pub link: String
}
impl  Versions {
    pub fn parse(data: &str) -> serde_json::Result<Vec<Self>> {
        serde_json::from_str(data)
    }
}