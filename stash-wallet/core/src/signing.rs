use rust_secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use ed25519_dalek::{Keypair as Ed25519Keypair, Signature as Ed25519Signature, Signer};
use schnorr_fun::fun::{marker::PhantomData, Point, Scalar, G};
use schnorr_fun::Schnorr;
use serde::{Deserialize, Serialize};
use hex;

#[derive(Serialize, Deserialize, Debug)]
pub enum Blockchain {
    Ethereum,
    Bitcoin,
    Solana,
    Tron,
    Bsc,
    Polygon,
    Avalanche,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub to: String,
    pub amount: String,
    pub chain_id: u64,
    pub nonce: Option<u64>,
    pub gas_price: Option<String>,
    pub data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedTransaction {
    pub tx: Transaction,
    pub signature: String,
    pub public_key: String,
    pub signed_raw: String,
}

pub fn sign_transaction_ecdsa(
    secret_key: &[u8],
    transaction: &Transaction,
) -> Result<SignedTransaction, String> {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(secret_key).map_err(|_| "Geçersiz özel anahtar")?;

    let message_hash = generate_message_hash(transaction)?;
    let msg = Message::from_slice(&message_hash).map_err(|_| "Mesaj hatası")?;

    let sig = secp.sign(&msg, &sk);
    let pub_key = PublicKey::from_secret_key(&secp, &sk);

    Ok(SignedTransaction {
        tx: transaction.clone(),
        signature: sig.to_string(),
        public_key: pub_key.to_string(),
        signed_raw: format!("0x{}", hex::encode(sig.serialize_der())),
    })
}

fn generate_message_hash(tx: &Transaction) -> Result<[u8; 32], String> {
    use sha2::{Digest, Sha256};

    let input = format!("{}{}{}", tx.to, tx.amount, tx.chain_id);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(result.as_slice());
    Ok(hash)
}