use super::dirs::dir_config_create;
use super::error::ConfigError;

extern crate config;
use self::config::reader::from_file;
use self::config::types::{Config, SettingsList};

use std::fs::metadata;

pub fn load_config(app: &str, file: &str) -> Result<Config, ConfigError> {
    let mut path_buf = try!(dir_config_create(app));
    path_buf.push(file);
    let path = path_buf.as_path();

    if metadata(path).ok().map_or(false, |m| m.is_file()) {
        let config = try!(from_file(path));
        Ok(config)
    } else {
        Ok(Config::new(SettingsList::new()))
    }
}
