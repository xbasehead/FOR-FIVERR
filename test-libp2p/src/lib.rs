use libp2p::identity;

pub fn generate_keypair() {
    let _keypair = identity::Keypair::generate_ed25519();
}
