use std::any::type_name;

pub mod crypt;
pub mod fs;
pub mod get;
pub mod net;
pub mod status;

#[cfg(test)]
mod tests;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
