use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use wasm_bindgen::prelude::*;
use bech32::{ToBase32, Bech32Writer};

const HRP_MAINNET: &str = "bc";
const HRP_TESTNET: &str = "tb";

pub fn sha256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

pub fn ripemd160(input: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

#[wasm_bindgen]
pub fn generate_bech32_address(public_key_hex: &str) -> String {
    // Convert hex to bytes
    let public_key_bytes = hex::decode(public_key_hex).expect("Invalid hex string");

    // Step 1: SHA-256 hashing of the public key
    let sha256_hash = sha256(&public_key_bytes);

    // Step 2: RIPEMD-160 hashing of the SHA-256 hash
    let ripemd160_hash = ripemd160(&sha256_hash);

    // Step 3: Bech32 encoding with witness version 0
    let bech32_address = encode_segwit_address(HRP_MAINNET, 0, &ripemd160_hash)
        .expect("Error encoding Bech32 address");

    bech32_address
}

pub fn encode_segwit_address(hrp: &str, version: u8, program: &[u8]) -> Result<String, bech32::Error> {
    let mut bech32_writer = Bech32Writer::new(hrp, bech32::Variant::Bech32)?;
    bech32_writer.write_u5(bech32::u5::try_from_u8(version)?)?;
    bech32_writer.write_all(&program.to_base32())?;
    bech32_writer.finalize()
}

