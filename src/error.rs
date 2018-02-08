use std::io;
use std::num;
use std::error;
use std::fmt;

pub enum MaeveError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for MaeveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaeveError::Io(ref err) => write!(f, "IO Error: {}", err),
            MaeveError::Parse(ref err) => write!(f, "Parse Error: {}", err),
        }
    }
}
