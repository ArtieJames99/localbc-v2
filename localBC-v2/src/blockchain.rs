use super::Block;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "".to_string());
        Blockchain { chain: vec![genesis_block] }
    }

    pub fn add_block_with_file(&mut self, file_path: &str) {
        let file_hash = file_hash(file_path);
        let file_name = std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap();
        let file_size = std::fs::metadata(file_path).unwrap().len();
    
        let data = format!("File: {}, Size: {}", file_name, file_size);
        let previous_block = self.chain.last().unwrap().clone();
        let index = previous_block.index + 1;
        let block = Block::new(index, data, previous_block.hash, file_hash);
        self.chain.push(block);
    }
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }

        true
    }
}
