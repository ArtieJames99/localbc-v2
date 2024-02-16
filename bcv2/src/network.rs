use libp2p::{
    core::{
        self,
        connection::ConnectionId,
        Multiaddr,
        transport::ListenerId,
        PeerId,
    },
    gossipsub::{
        Gossipsub,
        Topic,
    },
    identify::{
        Identify,
        IdentifyConfig,
    },
    swarm::{
        Swarm,
        SwarmBuilder,
    },
    tcp::TokioTcpConfig,
};

pub struct Network {
    pub swarm: Swarm<(Identify, Gossipsub)>,
    pub peers: Vec<PeerId>,
}

impl Network {
    pub async fn new(app: &mut App, local_key: &str) -> Self {
        let listener_id = ListenerId(0);
        let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .boxed();
        let mut behaviour = Gossipsub::new(
            Topic::new("blockchain"),
            vec![Topic::new("blockchain")],
            Default::default(),
        );
        let mut identify_config = IdentifyConfig::new(local_key.parse().unwrap());
        identify_config.listen_addr = Some(local_key.parse().unwrap());
        let identify = Identify::new(identify_config);
        let mut swarm = SwarmBuilder::new(transport, behaviour, identify)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build(local_key.parse().unwrap())
            .await
            .expect("can start swarm");
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).await.unwrap();
        Self {
            swarm,
            peers: vec![],
        }
    }

    pub async fn broadcast_block(&mut self, block: &Block) {
        let block_json = serde_json::to_string(block).unwrap();
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(Topic::new("blockchain"), block_json.as_bytes())
            .await
            .unwrap();
    }

    pub async fn handle_message(&mut self, msg: &[u8], peer_id: &PeerId) {
        let block: Block = serde_json::from_slice(msg).unwrap();
        self.app.try_add_block(block);
        info!("block received from {}", peer_id);
    }

    pub async fn connect_to_peer(&mut self, peer_addr: Multiaddr) {
        self.swarm
            .dial(peer_addr)
            .await
            .expect("can dial to peer");
    }

    pub async fn get_peers(&mut self) {
        self.peers = self
            .swarm
            .listeners()
            .iter()
            .filter_map(|listener| {
                if let ListenerId(id) = listener.id {
                    if id == listener_id {
                        return None;
                    }
                }
                Some(listener.peer_id().clone())
            })
            .collect();
    }
}