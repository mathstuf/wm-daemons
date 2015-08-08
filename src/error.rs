extern crate config;
use self::config::error::ConfigError;

use std::fmt;
use std::error::Error;

use self::WrapConfigError::CError;

#[derive(Debug)]
pub enum WrapConfigError {
    CError(String),
}

impl fmt::Display for WrapConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));
        Ok(())
    }
}

impl Error for WrapConfigError {
    fn description(&self) -> &str {
        match *self {
            CError(ref desc) => &(*desc)[..],
        }
    }
}

impl From<ConfigError> for WrapConfigError {
    fn from(config: ConfigError) -> WrapConfigError {
        let default_desc = format!("unknown parse error");
        CError(format!("{:?} {}: {}", config.kind, config.desc, config.detail.unwrap_or(default_desc)))
    }
}
