extern crate xdg_basedir;
use error::FsError;

use std::path::PathBuf;

pub fn config_dir(name: &str) -> Result<PathBuf, FsError> {
    let mut path: PathBuf = try!(xdg_basedir::get_config_home());
    path.push(name);
    Ok(path)
}
