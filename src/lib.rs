use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use wasm_bindgen::prelude::*;

const BECH32_CHARS: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
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

pub fn bech32_encode(hrp: &str, data: &[u8]) -> String {
    let mut result: Vec<u8> = vec![];
    let mut combined: Vec<u8> = Vec::new();

    // Step 5: Add the witness version byte
    combined.push(0); // Version 0

    // Append data to the combined vector
    for byte in data {
        // Convert each byte to 5-bit representation
        combined.push(*byte);
    }

    // Step 6: Compute the checksum
    let checksum = bech32_checksum(hrp, &combined);
    combined.extend(checksum);

    // Step 8: Map to Bech32 characters
    for chunk in combined.chunks(5) {
        let value = (chunk[0] >> 3) & 31; // Get the 5-bit value
        result.push(BECH32_CHARS[value as usize]);
    }

    // Step 9: Combine HRP, separator, and data
    format!("{}1{}", hrp, String::from_utf8(result).expect("Invalid UTF-8"))
}

fn bech32_checksum(hrp: &str, data: &[u8]) -> Vec<u8> {
    // Implement the checksum algorithm as per BIP 173
    let mut values: Vec<u8> = vec![];
    values.extend(hrp.chars().map(|c| c as u8));
    values.push(0); // Separator
    values.extend(data.iter());

    // Generate the polynomial and checksum
    // (For brevity, this implementation is omitted; you'd need to include the logic for generating
    // the checksum according to BIP 173 specifications.)

    // Return the calculated checksum as a Vec<u8>
    vec![0, 0, 0, 0, 0] // Placeholder: Replace with actual checksum values
}

#[wasm_bindgen]
pub fn generate_bech32_address(public_key_hex: &str) -> String {
    // Convert hex to bytes
    let public_key_bytes = hex::decode(public_key_hex).expect("Invalid hex string");

    // Step 1: SHA-256 hashing
    let sha256_hash = sha256(&public_key_bytes);

    // Step 2: RIPEMD-160 hashing
    let ripemd160_hash = ripemd160(&sha256_hash);

    // Step 3: Bech32 encoding
    let bech32_address = bech32_encode(HRP_MAINNET, &ripemd160_hash);

    bech32_address
}

