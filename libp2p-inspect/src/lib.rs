use libp2p_identity::{Keypair, PeerId};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

pub fn generate_keypair() -> (Keypair, PeerId) {
    let mut csprng = OsRng;
    let dalek_keypair: SigningKey = SigningKey::generate(&mut csprng);
    let mut secret_bytes = dalek_keypair.to_bytes();
    let secret_key = libp2p_identity::ed25519::SecretKey::try_from_bytes(&mut secret_bytes).expect("Failed to create secret key");
    let ed25519_keypair = libp2p_identity::ed25519::Keypair::from(secret_key);
    let keypair = Keypair::from(ed25519_keypair);
    let peer_id = keypair.public().to_peer_id();
    (keypair, peer_id)
}
