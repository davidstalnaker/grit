use std::fmt;
use std::io;

pub enum GritError {
    IoError(io::Error),
    NoGritDir,
    InvalidIndexFile,
}

impl fmt::Display for GritError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &GritError::IoError(ref inner) => inner.fmt(formatter),
            &GritError::NoGritDir => formatter.write_str("No grit directory found."),
            &GritError::InvalidIndexFile => formatter.write_str("The index is corrupt."),
        }
    }
}

impl From<io::Error> for GritError {
    fn from(err: io::Error) -> GritError {
        GritError::IoError(err)
    }
}
