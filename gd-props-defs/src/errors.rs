use core::fmt;

use ron::error::SpannedError;

#[derive(Debug)]
pub enum GdPropError {
    OpenFileRead,
    OpenFileWrite,
    HeaderDeserialize(SpannedError),
    HeaderSerialize,
    FileRead(std::io::Error),
    FileWrite(std::io::Error),
}

impl fmt::Display for GdPropError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GdPropError::OpenFileRead => write!(f, "can't open file for reading"),
            GdPropError::OpenFileWrite => write!(f, "can't open file for writing"),
            GdPropError::HeaderDeserialize(spanned) => {
                write!(f, "can't deserialize header: {}", spanned)
            }
            GdPropError::HeaderSerialize => write!(f, "can't serialize header"),
            GdPropError::FileRead(error) => write!(f, "can't read file: {}", error),
            GdPropError::FileWrite(error) => write!(f, "can't write to file: {}", error),
        }
    }
}
