use notblib::{crypt::aes_256::*, status};

/// Test AES-256 crypting
fn main() -> Result<(), std::io::Error> {
    let key = Key::gen();
    let data = b"Hello, world!";

    match encrypt(&key, data.to_vec()) {
        Ok(encrypted) => {
            status::success(format!(
                "Encrypted: {:?}",
                String::from_utf8_lossy(&encrypted)
            ));

            match decrypt(&key, &encrypted) {
                Ok(decrypted) => {
                    status::success(format!(
                        "Decrypted: {:?}",
                        String::from_utf8_lossy(&decrypted)
                    ));
                }
                Err(e) => {
                    status::fatal(format!("Decryption failed: {:?}", e));
                }
            }
        }
        Err(e) => {
            status::fatal(format!("Encryption failed: {:?}", e));
        }
    }
    Ok(())
}
