use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use randomizer::{Charset, Randomizer};
use std::error::Error;

pub struct Key {
    key_bytes: [u8; 32],
    nonce: [u8; 12],
}

impl Key {
    pub fn gen() -> Self {
        let key_bytes = {
            let bytes = Randomizer::new(256, Some(Charset::AnyByte))
                .bytes()
                .unwrap();
            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&bytes[0..32]);
            key_array
        };

        let nonce = {
            let bytes = Randomizer::new(12, Some(Charset::AnyByte)).bytes().unwrap();
            let mut nonce_array = [0u8; 12];
            nonce_array.copy_from_slice(&bytes[0..12]);
            nonce_array
        };

        Key { key_bytes, nonce }
    }

    pub fn bytes(&self) -> &[u8; 32] {
        &self.key_bytes
    }

    pub fn nonce(&self) -> &[u8; 12] {
        &self.nonce
    }
}

pub fn encrypt(key: &Key, data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256GcmSiv::new_from_slice(key.bytes())?;

    let nonce = Nonce::from_slice(key.nonce());

    let ciphertext = match cipher.encrypt(nonce, data.as_slice()) {
        Ok(ciphertext) => ciphertext,
        Err(err) => return Err(err.to_string().into()),
    };

    Ok(ciphertext)
}

pub fn decrypt(key: &Key, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256GcmSiv::new_from_slice(key.bytes())?;

    let nonce = Nonce::from_slice(key.nonce());

    let data = match cipher.decrypt(nonce, encrypted_data) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string().into()),
    };

    Ok(data)
}
