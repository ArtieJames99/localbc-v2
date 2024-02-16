use super::block;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;

fn file_hash(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(contents);
    let hash = base64::encode(hasher.finalize());

    hash
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    chain: Vec<block::Block>,
}