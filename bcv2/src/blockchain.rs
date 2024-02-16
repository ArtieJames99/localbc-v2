use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fmt::{self, Debug, Formatter};
use hex::FromHex;
use ring::digest;
use base64::{engine::general_purpose, Engine as _};
use libp2p::{Multiaddr, PeerId};
use libp2p::core::{ConnectedPoint, ConnectionId};
use libp2p::swarm::Swarm;
use libp2p::swarm::SwarmBuilder;
use libp2p::identify::Identify;
use libp2p::gossipsub::Gossipsub;
use libp2p::mdns::Mdns;
use libp2p::tcp::TokioTcpConfig;
use tokio::sync::RwLock;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub data_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub miner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileData {
    pub name: String,
    pub data: Vec<u8>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block {{\n  id: {},\n  timestamp: {},\n  previous_hash: {},\n  data_hash: {},\n  hash: {},\n  nonce: {},\n  miner: {}\n}}",
            self.id, self.timestamp, self.previous_hash, self.data_hash, self.hash, self.nonce, self.miner
        )
    }
}

impl Debug for HashRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "HashRate {{ hashes_per_second: {:.2} }}", self.hashes_per_second)
    }
}

pub struct BlockchainApp {
    pub blocks: RwLock<Vec<Block>>,
    pub peers: RwLock<Vec<PeerId>>,
}

impl BlockchainApp {
    pub fn new() -> Self {
        Self {
            blocks: RwLock::new(vec![
                Block::new(0, "genesis".to_string(), FileData { name: "genesis.txt".to_string(), data: vec![] }, "miner1".to_string()),
            ]),
            peers: RwLock::new(vec![]),
        }
    }

    pub async fn start(&self, listen_addr: Multiaddr) {
        let mut config = TokioTcpConfig::new();
        config.nodelay(true);

        let mut transport = libp2p::development_transport(config
````).await.expect("Failed to create transport");
transport.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
.expect("Failed to listen on address");

let local\_key = PeerId::from\_b58\_string("QmWATdJt1CaALzgjKfTpJtQaSgmLhQ2qcKz9oXbZo8XqoR").unwrap();
let mut swarm = SwarmBuilder::new(transport, local\_key, move || {
let mut config = Gossipsub::default();
config.heartbeat_interval = std::time::Duration::from\_secs(5);
config.heartbeat_grace_period = std::time::Duration::from\_secs(10);
config.message_limit = 100;
config.message_pool_limit = 100;
config.message_pool_retention_period = std::time::Duration::from\_secs(60);
config.max_transmit_size = 1024 * 1024;
config.topic_subscriptions = vec!["blockchain".to\_string()];

Gossipsub::with\_config(config)
})
.executor(Box::new(|fut| {
tokio::spawn(fut);
fut.await
}))
.build();

swarm.behaviour_mut().add\_our\_info(Identify::new("/blockchain/0.1".parse().unwrap()));
swarm.behaviour_mut().add\_middleware(Mdns::new());
swarm.behaviour_mut().add\_middleware(BlockchainBehaviour { app: self });

swarm.listen_on(listen\_addr).await.unwrap();

tokio::spawn(async move {
swarm.for\_each(|_stream, event| {
match event {
Event::Behaviour(event) => {
match event {
BehaviourEvent::OutboundSubstreamRequested {
substream_request,
..
} => {
let mut app = substream\_request.context().get::<BlockchainApp>().unwrap().write().await;
let blockchain = &mut *app;
let block = blockchain.blocks.read().await[0].clone();
let data = serde\_json::to\_vec(&block).unwrap();
let substream = substream\_request.substream();
substream.send(data).unwrap();
}
BehaviourEvent::InboundSubstreamOpened {
substream,
..
} => {
let mut app = substream.context().get::<BlockchainApp>().unwrap().write().await;
let mut blockchain = &mut *app;
let mut buffer = vec![];
substream.read\_to\_end(&mut buffer).await.unwrap();
let block: Block = serde\_json::from\_slice(&buffer).unwrap();
blockchain.blocks.write().await.push(block);
}
\_ => {}
}
}
\_ => {}
}
}).await;
});
}
}

struct BlockchainBehaviour {
app: &'static BlockchainApp,
}

impl Behaviour for BlockchainBehaviour {
type ConnectionHandler = ConnectionHandler<libp2p::gossipsub::Message, libp2p::gossipsub::Topic>;

fn new_connection(&mut self, _: &ConnectionId, _: &PeerId) -> Option<Self::ConnectionHandler> {
None
}

fn on_swarm\_event(&mut self, event: SwarmEvent) {
match event {
SwarmEvent::NewListenAddr { address, .. } => {
println!("Listening on {:?}", address);
}
SwarmEvent::Behaviour(event) => {
match event {
BehaviourEvent::PeerConnected { peer\_id, .. } => {
println!("Connected to {}", peer\_id);
let mut peers = self.app.peers.write().unwrap();
peers.push(peer\_id);
}
BehaviourEvent::PeerDisconnected { peer\_id, .. } => {
println!("Disconnected from {}", peer\_id);
let mut peers = self.app.peers.write().unwrap();
peers.retain(|id| *id != peer\_id);
}
\_ => {}
}
}
\_ => {}
}
}

fn on_connection_handler_event(
&mut self,
_: &ConnectionId,
event: ConnectionHandlerEvent<libp2p::gossipsub::Message, libp2p::gossipsub::Topic>,
) {
match event {
ConnectionHandlerEvent::Message { message, .. } => {
let mut app = self.app.clone();
tokio::spawn(async move {
match message {
libp2p::gossipsub::Message::Gossipsub(gossipsub\_message) => {
match gossipsub\_message.data {
Ok(data) => {
let block: Block = serde\_json::from\_slice(&data).unwrap();
let mut blocks = app.blocks.write().await;
blocks.push(block);
}
\_ => {}
}
\_ => {}
}
});
}
\_ => {}
}
}

fn connection_handler_closed(&mut self, _: &ConnectionId, _: ConnectionHandlerClosed) {}
}