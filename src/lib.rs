pub mod crypt;
pub mod fmt;
pub mod fs;
pub mod get;
pub mod net;
pub mod stat;

#[cfg(test)]
mod tests;

type AnyError = Box<dyn std::error::Error>;
