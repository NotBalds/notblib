use std::error::Error;
use toml;

pub fn from<T>(data: &dyn ToString) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let result: T = toml::from_str(&data.to_string()).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(result)
}

pub fn to<T>(data: T) -> Result<String, Box<dyn Error>>
where
    T: serde::ser::Serialize,
{
    Ok(toml::to_string(&data).map_err(|e| Box::new(e) as Box<dyn Error>)?)
}
