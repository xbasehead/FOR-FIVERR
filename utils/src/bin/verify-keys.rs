use libp2p_identity::{Keypair, PeerId};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    if input.trim().is_empty() {
        return Err("No input received from stdin".into());
    }

    // Parse the JSON
    let json: serde_json::Value = serde_json::from_str(&input)?;

    // Extract private key and peer ID
    let private_key_hex = json["private_key"]
        .as_str()
        .ok_or("Missing or invalid private_key field")?;
    let peer_id_str = json["peer_id"]
        .as_str()
        .ok_or("Missing or invalid peer_id field")?;

    // Decode private key
    let private_key_bytes = hex::decode(private_key_hex)?;
    let keypair = Keypair::ed25519_from_bytes(private_key_bytes)?;
    let derived_peer_id = keypair.public().to_peer_id();

    // Parse the provided peer ID
    let provided_peer_id: PeerId = peer_id_str.parse()?;

    // Verify the peer ID matches the one derived from the private key
    if derived_peer_id != provided_peer_id {
        return Err("Peer ID does not match the private key".into());
    }

    println!("Keys are valid! Peer ID: {}", provided_peer_id);
    Ok(())
}
