#[test]
fn status_test() {
    use crate::status;

    status::process("Test process status");
    status::success("Test success status");
    status::warning("Test warning status");
    status::error("Test error status");
    status::fatal("Test fatal status");
}

#[test]
fn aes256_test() -> Result<(), super::AnyError> {
    use crate::crypt::aes_256::*;

    let key = Key::gen();
    let data = b"Hello, world!";

    let encrypted_data = encrypt(&key, data.to_vec())?;
    let decrypted_data = decrypt(&key, &encrypted_data)?;
    assert_eq!(decrypted_data, data);
    Ok(())
}
