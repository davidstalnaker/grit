use std::fmt;
use std::io;

pub enum GritError {
    IoError,
    NoGritDir,
    InvalidIndexFile,
}

impl fmt::Display for GritError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        return formatter.write_str(match self {
            &GritError::IoError => "An IO error occured",
            &GritError::NoGritDir => "No grit directory found.",
            &GritError::InvalidIndexFile => "The index is corrupt."
        });
    }
}

impl From<io::Error> for GritError {
    fn from(_: io::Error) -> GritError {
        GritError::IoError
    }
}
