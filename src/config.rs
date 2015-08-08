use super::dirs::dir_config_create;
use super::error::WrapConfigError;

extern crate config;
use self::config::reader::{from_file, from_stream};
use self::config::types::{Config, SettingsList};

use std::error::Error;
use std::fs::metadata;
use std::io::stdin;
use std::path::Path;

// Fix this craziness. Waiting on https://github.com/filipegoncalves/rust-config/issues/2.
fn wrap_from_file(path: &Path) -> Result<Config, WrapConfigError> {
    Ok(try!(if path.to_str() == Some("-") {
        from_stream(&mut stdin())
    } else {
        from_file(path)
    }))
}

pub fn load_config(app: &str, file: &str) -> Result<Config, Box<Error>> {
    let mut path_buf = try!(dir_config_create(app));
    path_buf.push(file);

    load_config_path(path_buf.as_path())
}

pub fn load_config_path(path: &Path) -> Result<Config, Box<Error>> {
    if metadata(path).ok().map_or(false, |m| m.is_file()) {
        let config = try!(wrap_from_file(path));
        Ok(config)
    } else {
        Ok(Config::new(SettingsList::new()))
    }
}
