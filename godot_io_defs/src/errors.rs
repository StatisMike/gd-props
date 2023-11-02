use core::fmt;

use ron::error::SpannedError;

#[derive(Debug, Clone)]
pub enum GdRonError {
  OpenFileRead,
  OpenFileWrite,
  HeaderDeserialize(SpannedError),
  HeaderSerialize
}

impl fmt::Display for GdRonError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        GdRonError::OpenFileRead => write!(f, "Can't open file for reading"),
        GdRonError::OpenFileWrite => write!(f, "Can't open file for writing"),
        GdRonError::HeaderDeserialize(spanned) => write!(f, "Can't deserialize header: {}", spanned),
        GdRonError::HeaderSerialize => write!(f, "Can't serialize header"),
    }
  }
}