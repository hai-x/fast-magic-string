use std::{io, string};

use fms_sourcemap::error::SourcemapError;

#[derive(Debug)]
pub struct Error {
  pub err_type: FmsErrType,
  pub err_msg: Option<String>,
}

impl Default for Error {
  fn default() -> Self {
    Self {
      err_type: FmsErrType::Default,
      err_msg: None,
    }
  }
}

impl From<serde_json::Error> for Error {
  #[inline]
  fn from(_: serde_json::Error) -> Self {
    Error::new(FmsErrType::JSON)
  }
}

impl From<io::Error> for Error {
  #[inline]
  fn from(_: io::Error) -> Self {
    Error::new(FmsErrType::IO)
  }
}

impl From<SourcemapError> for Error {
  #[inline]
  fn from(err: SourcemapError) -> Self {
    match err {
      SourcemapError::Vlq(_) => Error::new(FmsErrType::Vlq),
      SourcemapError::Io(_) => Error::new(FmsErrType::IO),
      SourcemapError::FromUtf8Error(_) => Error::new(FmsErrType::StringFromUTF8),
    }
  }
}

impl From<string::FromUtf8Error> for Error {
  #[inline]
  fn from(_: string::FromUtf8Error) -> Self {
    Error::new(FmsErrType::StringFromUTF8)
  }
}

impl Error {
  pub fn new(err_type: FmsErrType) -> Self {
    Self {
      err_type,
      err_msg: None,
    }
  }
  pub fn from_reason(err_type: FmsErrType, reason: &str) -> Self {
    Self {
      err_type,
      err_msg: Some(String::from(reason)),
    }
  }
}

#[derive(Debug)]
pub enum FmsErrType {
  Default,
  Deprecated,
  Range,
  Overwrite,
  SplitChunk,
  Type,
  JSON,
  IO,
  Vlq,
  StringFromUTF8,
  Slice,
}
