extern crate xdg_basedir;

use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;

/// Creates a the configuration directory for the application and returns its path.
pub fn dir_config_create(name: &str) -> Result<PathBuf, Box<Error>> {
    let path = try!(dir_config(name));
    try!(create_dir_all(path.as_path()));
    Ok(path)
}

/// Returns the path to the configuration directory for the application.
pub fn dir_config(name: &str) -> Result<PathBuf, Box<Error>> {
    let mut path = try!(xdg_basedir::get_config_home());
    path.push(name);
    Ok(path)
}

/// Creates a the data directory for the application and returns its path.
pub fn dir_data_create(name: &str) -> Result<PathBuf, Box<Error>> {
    let path = try!(dir_data(name));
    try!(create_dir_all(path.as_path()));
    Ok(path)
}

/// Returns the path to the data directory for the application.
pub fn dir_data(name: &str) -> Result<PathBuf, Box<Error>> {
    let mut path = try!(xdg_basedir::get_data_home());
    path.push(name);
    Ok(path)
}
