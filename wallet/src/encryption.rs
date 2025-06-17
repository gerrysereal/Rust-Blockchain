use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::OsRng;
use bs58;

pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}

pub fn get_address(public_key: &PublicKey) -> String {
    bs58::encode(public_key.to_bytes()).into_string()
}