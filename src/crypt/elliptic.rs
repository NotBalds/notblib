use ecies::utils::generate_keypair;
pub use ecies::{decrypt, encrypt};

pub type SecretKey = [u8; 32];
pub type PublicKey = [u8; 65];

pub struct Key {
    pub secret: SecretKey,
    pub public: PublicKey,
}

pub fn gen_keys() -> Key {
    let (sk, pk) = generate_keypair();
    let (sk, pk) = (&sk.serialize(), &pk.serialize());
    Key {
        secret: *sk,
        public: *pk,
    }
}
