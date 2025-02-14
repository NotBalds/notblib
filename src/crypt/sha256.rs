use sha2::{Digest, Sha256};

pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&result[..]);
    hash_bytes
}

pub fn hash_str(input: &str) -> String {
    let hash_bytes = hash(input.as_bytes());
    hex::encode(hash_bytes)
}
