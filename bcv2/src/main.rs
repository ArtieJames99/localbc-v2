use blockchain::BlockchainApp;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
let app = BlockchainApp::new();
let listen_addr = "/ip4/0.0.0.0/tcp/8080".parse().unwrap();

app.start(listen_addr).await;
tokio::time::delay_for(Duration::from_secs(10)).await;

}