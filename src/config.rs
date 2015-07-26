extern crate xdg_basedir;
use std::path::PathBuf;
use self::xdg_basedir::error::Error;

pub fn config_dir(name: &str) -> Result<PathBuf, Error> {
    let mut path: PathBuf = try!(xdg_basedir::get_config_home());
    path.push(name);
    Ok(path)
}
