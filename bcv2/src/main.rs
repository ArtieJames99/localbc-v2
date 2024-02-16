
use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;
use pretty_env_logger::env_logger;
use chrono:Utc;

use jtg::blockchain::{App, Block};
use jtg::network::{Network, DIFFICULTY_PREFIX};

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = Arc::new(RwLock::new(App::new()));
    let local_key = "local_key".to_string();
    let mut network = Network::new(&mut app, &local_key).await;
    network.get_peers().await;
    info!("peers: {:?}", network.peers);
    let mut block = Block {
        id: 1,
        timestamp: Utc::now().timestamp(),
        previous_hash: String::from("genesis"),
        data: String::from("block 1"),
        nonce: 0,
        hash: String::from(""),
    };
    network.mine_block(&mut block, 4);
    network.broadcast_block(&block).await;
    loop {
        network.swarm.poll().await;
    }
}