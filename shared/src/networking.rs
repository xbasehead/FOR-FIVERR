use libp2p::{
    core::upgrade,
    noise,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp,
    yamux::YamuxConfig,
    Transport,
};
use libp2p::identity::Keypair;
use std::error::Error;

pub fn build_swarm(keypair: Keypair) -> Result<libp2p::Swarm<libp2p::swarm::dummy::Behaviour>, Box<dyn Error>> {
    let transport = tcp::async_io::Transport::new(tcp::Config::new())
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(keypair.clone()).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

    let behaviour = libp2p::swarm::dummy::Behaviour; // Placeholder; replace with your behaviour
    let swarm = SwarmBuilder::with_async_std_executor(
        transport,
        behaviour,
        keypair.public().to_peer_id(),
    ).build();

    Ok(swarm)
}
