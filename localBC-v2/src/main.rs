mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block_with_file("path/to/your/file.txt");
    blockchain.add_block_with_file("path/to/another/file.txt");

    println!("Blockchain is valid: {}", blockchain.is_valid());
}