use core::fmt;

#[derive(Debug, Clone)]
pub enum GdRonError {
  OpenFile,
  HeaderRead
}

impl fmt::Display for GdRonError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match self {
        GdRonError::OpenFile => "Can't open file for reading",
        GdRonError::HeaderRead => "Can't read header",
    })
  }
}