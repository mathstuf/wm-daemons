extern crate xdg_basedir;
use self::xdg_basedir::error::Error as XdgError;
use self::FsError::{XdgFsError, IoFsError};

use std::fmt;
use std::error::Error;
use std::io;

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
