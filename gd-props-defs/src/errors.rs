use core::fmt;

use ron::error::SpannedError;

#[derive(Debug, Clone)]
pub enum GdPropError {
    OpenFileRead,
    OpenFileWrite,
    HeaderDeserialize(SpannedError),
    HeaderSerialize,
}

impl fmt::Display for GdPropError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GdPropError::OpenFileRead => write!(f, "Can't open file for reading"),
            GdPropError::OpenFileWrite => write!(f, "Can't open file for writing"),
            GdPropError::HeaderDeserialize(spanned) => {
                write!(f, "Can't deserialize header: {}", spanned)
            }
            GdPropError::HeaderSerialize => write!(f, "Can't serialize header"),
        }
    }
}
