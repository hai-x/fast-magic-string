use std::{io, string};

use vlq;

#[derive(Debug)]
pub enum SourcemapError {
  Io(io::Error),

  Vlq(vlq::Error),
  FromUtf8Error(string::FromUtf8Error),
}

impl From<io::Error> for SourcemapError {
  fn from(err: io::Error) -> Self {
    SourcemapError::Io(err)
  }
}

impl From<vlq::Error> for SourcemapError {
  fn from(err: vlq::Error) -> Self {
    SourcemapError::Vlq(err)
  }
}

impl From<string::FromUtf8Error> for SourcemapError {
  fn from(err: string::FromUtf8Error) -> Self {
    SourcemapError::FromUtf8Error(err)
  }
}
