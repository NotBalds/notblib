use std::{
    fs::{create_dir, create_dir_all, File},
    io::Error,
    path::Path,
};

pub fn file(path: &Path) -> Result<(), Error> {
    File::create_new(path)?;
    Ok(())
}

pub fn dir(path: &Path) -> Result<(), Error> {
    create_dir(path)?;
    Ok(())
}

pub fn dir_all(path: &Path) -> Result<(), Error> {
    create_dir_all(path)?;
    Ok(())
}
