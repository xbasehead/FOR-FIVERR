use libp2p::{
    gossipsub::{self, MessageAuthenticity, IdentTopic},
    ping::Behaviour as PingBehaviour,
    Swarm, SwarmBuilder, Multiaddr, PeerId, identity,
    core::transport::Transport,
    swarm::NetworkBehaviour,
};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum CustomEvent {
    Gossipsub(gossipsub::Event),
    Ping(libp2p::ping::Event),
}

impl From<gossipsub::Event> for CustomEvent {
    fn from(event: gossipsub::Event) -> Self {
        CustomEvent::Gossipsub(event)
    }
}

impl From<libp2p::ping::Event> for CustomEvent {
    fn from(event: libp2p::ping::Event) -> Self {
        CustomEvent::Ping(event)
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "CustomEvent")]
pub struct CombinedBehaviour {
    gossipsub: gossipsub::Behaviour,
    ping: PingBehaviour,
}

impl CombinedBehaviour {
    pub fn subscribe(&mut self, topic: &IdentTopic) -> bool {
        self.gossipsub.subscribe(topic).unwrap_or(false)
    }

    pub fn publish(&mut self, topic: IdentTopic, data: impl Into<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        let data = data.into();
        println!("Publishing to topic {} with message size {} bytes", topic, data.len());
        let mesh_peers: Vec<_> = self.gossipsub.mesh_peers(&topic.hash()).collect();
        if mesh_peers.is_empty() {
            println!("No mesh peers for topic {}", topic);
        } else {
            println!("Mesh peers for topic {}: {:?}", topic, mesh_peers);
        }
        self.gossipsub
            .publish(topic.clone(), data)
            .map_err(|e| {
                println!("Publish failed: {:?}", e);
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Gossipsub publish error: {:?}", e))) as Box<dyn std::error::Error>
            })?;
        println!("Successfully published message to topic {}", topic);
        Ok(())
    }

    pub fn mesh_peers(&self, topic: &IdentTopic) -> impl Iterator<Item = &PeerId> {
        self.gossipsub.mesh_peers(&topic.hash())
    }
}

pub struct Network {
    pub swarm: Swarm<CombinedBehaviour>,
}

impl Network {
    pub fn publish_message(&mut self, topic: IdentTopic, data: impl Into<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        self.swarm.behaviour_mut().publish(topic, data)
    }
}

impl fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Network")
            .field("peer_id", &self.swarm.local_peer_id())
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub dial_addrs: Vec<String>,
}

pub async fn setup_network(config: NetworkConfig) -> Result<Network, Box<dyn std::error::Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer ID: {}", local_peer_id);

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .mesh_n(4)
        .mesh_n_low(3)
        .mesh_n_high(5)
        .build()
        .map_err(|e| {
            println!("Gossipsub config error: {}", e);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Gossipsub config error: {}", e)))
        })?;

    let gossipsub = gossipsub::Behaviour::new(MessageAuthenticity::Signed(local_key.clone()), gossipsub_config)
        .map_err(|e| {
            println!("Gossipsub init error: {}", e);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Gossipsub init error: {}", e)))
        })?;

    let ping = PingBehaviour::new(libp2p::ping::Config::new());

    let transport = libp2p::tcp::tokio::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(
            libp2p::noise::Config::new(&local_key)
                .map_err(|e| {
                    println!("Noise auth config error: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, format!("Noise auth config error: {}", e))
                })?,
        )
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_other_transport(|_| Ok(transport))
        .map_err(|e| {
            println!("Transport setup error: {}", e);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Transport setup error: {}", e)))
        })?
        .with_behaviour(|_| CombinedBehaviour { gossipsub, ping })
        .map_err(|e| {
            println!("Behaviour setup error: {}", e);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Behaviour setup error: {}", e)))
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let listen_addr: Multiaddr = config.listen_addr.parse()
        .map_err(|e| {
            println!("Listen address parse error: {}", e);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Listen address parse error: {}", e)))
        })?;
    swarm.listen_on(listen_addr)
        .map_err(|e| {
            println!("Listen error: {}", e);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Listen error: {}", e)))
        })?;

    Ok(Network { swarm })
}