use protobuf;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MaeveError {
    Exit,
    Io(io::Error),
    Load(io::Error),
    Parse,
    Proto(protobuf::ProtobufError),
    Write(io::Error),
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
            MaeveError::Io(ref err) => write!(f, "IO Error: {}", err),
            MaeveError::Write(ref err) => write!(f, "Write Error: {}", err),
            MaeveError::Load(ref err) => write!(f, "Load Error: {}", err),
            MaeveError::Proto(ref err) => write!(f, "Proto Error: {}", err),
            MaeveError::Parse => write!(f, "Error parsing input."),
            MaeveError::Exit => write!(f, "We look forward to seeing you again."),
            MaeveError::WriteHistory => write!(f, "Error with .history.txt file"),
        }
    }
}

impl error::Error for MaeveError {
    fn description(&self) -> &str {
        match *self {
            MaeveError::Io(ref err) => err.description(),
            MaeveError::Write(ref err) => err.description(),
            MaeveError::Load(ref err) => err.description(),
            MaeveError::Proto(ref err) => err.description(),
            MaeveError::Parse => "Bad input",
            MaeveError::Exit => "Exiting",
            MaeveError::WriteHistory => "File error: .history.txt",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MaeveError::Io(ref err) => Some(err),
            MaeveError::Write(ref err) => Some(err),
            MaeveError::Load(ref err) => Some(err),
            MaeveError::Proto(ref err) => Some(err),
            MaeveError::Parse => None,
            MaeveError::Exit => None,
            MaeveError::WriteHistory => None,
        }
    }
}
