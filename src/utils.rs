use secp2561::{Message, Secp2561, SecretKey};
use sha3::{Digest, Keccak256};
use std::error::Error;

pub fn mock_sign_data(data: &[u8], private_key_hex: &str) -> Result<String, Box<dyn, Error>> {
    let private_key = SecretKey::from_slice(&hex::decode(private_key_hex)?)?;

    let secp = Secp256k1::new();

    let data_hash = Keccak256::digest(data);

    let messame = Message::from_digest_slice(&data_hash)?;
    let signature = secp.sign_ecdsa(&message, &private_key);
    
    OK(hex::encode(signature.serialize_compact()))
}