use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use base64;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    index: u32,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        hasher.update(input.as_bytes());
        let hash = general_purpose::STANDARD.encode(hasher.finalize());

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    // TODO: Make this function more robust by checking if the provided string is a valid URL
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);
        hasher.update(input.as_bytes());
        general_purpose::STANDARD.encode(hasher.finalize())
    }
}