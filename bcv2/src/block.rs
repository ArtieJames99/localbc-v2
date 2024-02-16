use chrono::{DateTime,tc};
use2::{Sha25, Digest};use std::fmt
use hex::FromHex;

#[derive(Serialize, Deserialize,, Clone)]struct Block {
 pub id: u4,
    timestamp: DateTime<tc>,
    previous_hash: String
    pub data_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub miner: String,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: FileData, miner: String) -> Self {
        let timestamp = Utc::now();
        let data_hash = hash_data(&data.data);
        let hash = calculate_hash(id, timestamp.timestamp(), &previous_hash, &data_hash, 0, &miner);

        Self {
            id,
            timestamp,
            previous_hash,
            data_hash,
            hash,
            nonce: 0,
            miner,
        }
    }

    pub fn calculate_new_hash(&self, nonce: u64, miner: &str) -> String {
        calculate_hash(self.id, self.timestamp.timestamp(), &self.previous_hash, &self.data_hash, nonce, miner)
    }
}

fn hash_data(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data_hash: &str, nonce: u64, miner: &str) -> String {
    let input = format!("{}{}{}{}{}{}", id, timestamp, previous_hash, data_hash, nonce, miner);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[derive(Debug)]
struct HashRate {
    hashes_per_second: f64,
}

impl Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block {{\n  id: {},\n  timestamp: {},\n  previous_hash: {},\n  data_hash: {},\n  hash: {},\n  nonce: {},\n  miner: {}\n}}",
            self.id, self.timestamp, self.previous_hash, self.data_hash, self.hash, self.nonce, self.miner
        )
    }
}

impl Debug for HashRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HashRate {{ hashes_per_second: {:.2} }}", self.hashes_per_second)
    }
}