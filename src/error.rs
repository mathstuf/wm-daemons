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
            // XXX: trim -> as_str
            CError(ref desc) => (*desc).trim(),
        }
    }
}

impl From<ConfigError> for WrapConfigError {
    fn from(config: ConfigError) -> WrapConfigError {
        // FIXME: Wow, the unstable APIs...
        let mut default_desc = String::new();
        default_desc.push_str("unknown parse error");
        let desc = config.detail.unwrap_or(default_desc);
        let mut err_string = String::new();
        err_string.push_str(config.desc);
        err_string.push_str(": ");
        // XXX: trim -> as_str
        err_string.push_str(desc.trim());
        CError(err_string)
    }
}
