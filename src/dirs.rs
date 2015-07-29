use super::error::FsError;

extern crate xdg_basedir;

use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn dir_config_create(name: &str) -> Result<PathBuf, FsError> {
    let path = try!(dir_config(name));
    try!(create_dir_all(path.as_path()));
    Ok(path)
}

pub fn dir_config(name: &str) -> Result<PathBuf, FsError> {
    let mut path = try!(xdg_basedir::get_config_home());
    path.push(name);
    Ok(path)
}

pub fn dir_data_create(name: &str) -> Result<PathBuf, FsError> {
    let path = try!(dir_data(name));
    try!(create_dir_all(path.as_path()));
    Ok(path)
}

pub fn dir_data(name: &str) -> Result<PathBuf, FsError> {
    let mut path = try!(xdg_basedir::get_data_home());
    path.push(name);
    Ok(path)
}
