use fast_magic_string::error::{Error, FmsErrType};

#[inline]
pub fn to_napi_error(err: Error) -> napi::Error {
  let mut reason = String::from("<rust-magic-string> ");
  match err.err_type {
    FmsErrType::Default => {
      reason.push_str("Default error (not expected to occur)");
    }
    FmsErrType::Deprecated => {
      reason.push_str("Deprecated api error");
    }
    FmsErrType::Range => {
      reason.push_str("Index out of range");
    }
    FmsErrType::Overwrite => {
      reason.push_str("Overwrite error");
    }
    FmsErrType::Type => {
      reason.push_str("TypeError");
    }
    FmsErrType::SplitChunk => {
      reason.push_str("Split chunk error");
    }
    FmsErrType::JSON => {
      reason.push_str("Json serialize error");
    }
    FmsErrType::IO => {
      reason.push_str("IO error");
    }
    FmsErrType::Vlq => {
      reason.push_str("Vlq encode error");
    }
    FmsErrType::StringFromUTF8 => {
      reason.push_str("String from utf-8 error");
    }
    FmsErrType::Slice => {
      reason.push_str("Slice error");
    }
  }
  reason.push_str(": ");
  reason.push_str(err.err_msg.unwrap_or_default().as_str());
  napi::Error::new(napi::Status::GenericFailure, reason)
}
