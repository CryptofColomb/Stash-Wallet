use bip39::{Language, Mnemonic, MnemonicType};
use bip32utils::BIP32ECDSA;
use hex;

pub fn generate_seed_phrase() -> (Mnemonic, String) {
    let mnemonic_type = MnemonicType::Words12;
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    let seed = mnemonic.to_seed("");
    (mnemonic, seed.to_hex())
}

pub fn derive_key_from_seed(seed: &[u8]) -> BIP32ECDSA {
    BIP32ECDSA::from_seed(seed).unwrap()
}