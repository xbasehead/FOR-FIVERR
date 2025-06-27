use libp2p_identity::Keypair;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();

    let output = serde_json::json!({
        "private_key": hex::encode(keypair.to_protobuf_encoding().unwrap()),
        "peer_id": peer_id.to_string(),
    });

    // Write to file as a side effect
    let mut file = File::create("node_keys.json")?;
    file.write_all(serde_json::to_string_pretty(&output)?.as_bytes())?;

    // Output JSON to stdout for piping
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}
