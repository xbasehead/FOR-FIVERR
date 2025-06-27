use libp2p::{
    gossipsub::{self, IdentTopic, MessageAuthenticity},
    swarm::SwarmEvent,
    Multiaddr, PeerId, Swarm,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Network {
    pub swarm: Swarm<gossipsub::Behaviour>,
    pub topic: IdentTopic,
    pub receiver: mpsc::Receiver<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: &'static str,
    pub dial_addrs: Vec<&'static str>,
}

pub async fn setup_network(config: NetworkConfig) -> Result<Network, Box<dyn Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let mut swarm = Swarm::new(
        libp2p::tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key)?)
            .multiplex(libp2p::yamux::Config::default())
            .boxed(),
        gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(local_key),
            gossipsub::ConfigBuilder::default().build()?,
        )?,
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor(),
    );

    swarm.listen_on(config.listen_addr.parse::<Multiaddr>()?)?;

    for addr in config.dial_addrs {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
    }

    let topic = IdentTopic::new("test-net");
    swarm.behaviour_mut().subscribe(&topic)?;

    let (sender, receiver) = mpsc::channel(100);
    let network = Network {
        swarm,
        topic,
        receiver,
    };

    tokio::spawn(async move {
        while let Some(message) = network.receiver.recv().await {
            if let Err(e) = network.swarm.behaviour_mut().publish(network.topic.clone(), message) {
                eprintln!("Publish error: {:?}", e);
            }
        }
    });

    Ok(network)
}

impl Network {
    pub fn publish(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        self.swarm
            .behaviour_mut()
            .publish(self.topic.clone(), message)?;
        Ok(())
    }
}

pub type NetworkEvent = SwarmEvent<gossipsub::Event>;
