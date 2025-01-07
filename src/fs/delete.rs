use std::{
    fs::{remove_dir, remove_dir_all, remove_file},
    io::Error,
    path::Path,
};

pub fn file(path: &Path) -> Result<(), Error> {
    remove_file(path)?;
    Ok(())
}

pub fn dir(path: &Path) -> Result<(), Error> {
    remove_dir(path)?;
    Ok(())
}

pub fn dir_all(path: &Path) -> Result<(), Error> {
    remove_dir_all(path)?;
    Ok(())
}
