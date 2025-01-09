pub use serde::{Deserialize, Serialize};
pub use serde_json::Error;

pub fn from<T: for<'de> Deserialize<'de>>(json_str: &str) -> Result<T, Error> {
    serde_json::from_str(json_str)
}

pub fn to<T: Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value)
}
