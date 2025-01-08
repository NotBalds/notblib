pub mod crypt;
pub mod fs;
pub mod get;
pub mod net;
pub mod status;

#[cfg(test)]
mod tests;

type AnyError = Box<dyn std::error::Error>;
