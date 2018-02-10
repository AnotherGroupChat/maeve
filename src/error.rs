use protobuf;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MaeveError {
    Exit,
    Io(io::Error),
    Load,
    Parse,
    Proto(protobuf::ProtobufError),
    Write,
    WriteHistory,
}

impl From<io::Error> for MaeveError {
    fn from(err: io::Error) -> MaeveError {
        MaeveError::Io(err)
    }
}

impl From<protobuf::ProtobufError> for MaeveError {
    fn from(err: protobuf::ProtobufError) -> MaeveError {
        MaeveError::Proto(err)
    }
}

impl fmt::Display for MaeveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaeveError::Exit => {
                write!(f, "We look forward to seeing you again!")
            }
            MaeveError::Io(ref err) => write!(f, "IO Error: {}", err),
            MaeveError::Load => write!(f, "Load Error!"),
            MaeveError::Parse => write!(f, "Error parsing input!"),
            MaeveError::Proto(ref err) => write!(f, "Proto Error: {}", err),
            MaeveError::Write => write!(f, "Write Error!"),
            MaeveError::WriteHistory => {
                write!(f, "Error writing .history.txt file!")
            }
        }
    }
}

impl error::Error for MaeveError {
    fn description(&self) -> &str {
        match *self {
            MaeveError::Exit => "Exiting",
            MaeveError::Io(ref err) => err.description(),
            MaeveError::Load => "Error loading file",
            MaeveError::Parse => "Bad input",
            MaeveError::Proto(ref err) => err.description(),
            MaeveError::Write => "Failed write",
            MaeveError::WriteHistory => "Failed write to .history.txt",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MaeveError::Exit => None,
            MaeveError::Io(ref err) => Some(err),
            MaeveError::Load => None,
            MaeveError::Parse => None,
            MaeveError::Proto(ref err) => Some(err),
            MaeveError::Write => None,
            MaeveError::WriteHistory => None,
        }
    }
}
