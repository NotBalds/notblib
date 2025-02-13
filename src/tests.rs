#[test]
fn stat_test() {
    use crate::stat;

    stat::process("Test process status");
    stat::success("Test success status");
    stat::warning("Test warning status");
    stat::error("Test error status");
    stat::fatal("Test fatal status");
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
