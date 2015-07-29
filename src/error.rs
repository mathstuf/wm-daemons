extern crate xdg_basedir;
use self::xdg_basedir::error::Error as XdgError;

extern crate config;
use self::config::error::ConfigError as ParseError;

use std::fmt;
use std::error::Error;
use std::io;

use self::FsError::{XdgFsError, IoFsError};
use self::ConfigError::{FsConfigError, ParseConfigError};

#[derive(Debug)]
pub enum FsError {
    XdgFsError(XdgError),
    IoFsError(io::Error),
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));
        Ok(())
    }
}

impl Error for FsError {
    fn description(&self) -> &str {
        match *self {
            XdgFsError(ref xdg) => (*xdg).description(),
            IoFsError(ref io) => (*io).description(),
        }
    }
}

impl From<XdgError> for FsError {
    fn from(xdg: XdgError) -> FsError {
        XdgFsError(xdg)
    }
}

impl From<io::Error> for FsError {
    fn from(io: io::Error) -> FsError {
        IoFsError(io)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    FsConfigError(FsError),
    ParseConfigError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));
        Ok(())
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            FsConfigError(ref fs) => (*fs).description(),
            // XXX: trim -> as_str
            ParseConfigError(ref desc) => (*desc).trim(),
        }
    }
}

impl From<FsError> for ConfigError {
    fn from(fs: FsError) -> ConfigError {
        FsConfigError(fs)
    }
}

impl From<ParseError> for ConfigError {
    fn from(config: ParseError) -> ConfigError {
        // FIXME: Wow, the unstable APIs...
        let mut default_desc = String::new();
        default_desc.push_str("unknown parse error");
        let desc = config.detail.unwrap_or(default_desc);
        let mut err_string = String::new();
        err_string.push_str(config.desc);
        err_string.push_str(": ");
        // XXX: trim -> as_str
        err_string.push_str(desc.trim());
        ParseConfigError(err_string)
    }
}
