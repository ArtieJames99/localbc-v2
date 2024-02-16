mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First transaction".to_string());
    blockchain.add_block("Second transaction".to_string());

    println!("Blockchain is valid: {}", blockchain.is_valid());
}