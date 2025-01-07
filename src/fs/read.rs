use {
    std::{
        error::Error as ErrorInterface,
        fs::{read, read_to_string},
        io::Error,
        path::Path,
    },
    toml,
};

pub fn bytes(path: &Path) -> Result<Vec<u8>, Error> {
    read(path)
}

pub fn string(path: &Path) -> Result<String, Error> {
    read_to_string(path)
}

pub fn from_toml<T>(path: &Path) -> Result<T, Box<dyn ErrorInterface>>
where
    T: serde::de::DeserializeOwned,
{
    let data = string(path).map_err(|e| Box::new(e) as Box<dyn ErrorInterface>)?;
    let result: T =
        toml::de::from_str(&data).map_err(|e| Box::new(e) as Box<dyn ErrorInterface>)?;
    Ok(result)
}
