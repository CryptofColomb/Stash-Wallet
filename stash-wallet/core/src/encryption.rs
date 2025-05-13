use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use zeroize::Zeroize;
use rand::RngCore;
use hex;

pub fn generate_aes_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

pub fn encrypt_data(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = generate_nonce();
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), data)
        .map_err(|e| e.to_string())?;
    Ok([nonce, ciphertext].concat())
}

pub fn decrypt_data(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(key.into());

    if data.len() < 12 {
        return Err("Geçersiz veri uzunluğu".to_string());
    }

    let (nonce, ciphertext) = data.split_at(12);

    cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| e.to_string())
}

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}