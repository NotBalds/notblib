use std::{error::Error, fs::write, io::Result as SRes, path::Path};

pub fn bytes(path: &Path, data: Vec<u8>) -> SRes<()> {
    write(path, data)?;
    Ok(())
}

pub fn str<D: AsRef<str>>(path: &Path, data: D) -> SRes<()> {
    write(path, data.as_ref())?;
    Ok(())
}

pub fn to_toml<T>(path: &Path, value: &T) -> Result<(), Box<dyn Error>>
where
    T: serde::ser::Serialize,
{
    let data = toml::to_string(value).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    str(path, data).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(())
}
