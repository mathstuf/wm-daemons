extern crate xdg_basedir;
use error::FsError;

use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn config_dir_create(name: &str) -> Result<PathBuf, FsError> {
    let path: PathBuf = try!(config_dir(name));
    try!(create_dir_all(path.as_path()));
    Ok(path)
}

pub fn config_dir(name: &str) -> Result<PathBuf, FsError> {
    let mut path: PathBuf = try!(xdg_basedir::get_config_home());
    path.push(name);
    Ok(path)
}
