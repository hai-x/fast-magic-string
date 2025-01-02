use std::fmt::{Debug, Error};

mod napi;

pub struct FmsRegex {
  pub flags: String,
  pub source: String,
}

impl Debug for FmsRegex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FmsRegex")
      .field("flags", &self.flags)
      .field("source", &self.source)
      .finish()
  }
}

impl FmsRegex {
  pub fn new(&self, expr: &str) -> Result<Self, Error> {
    Self::with_flags(expr, "")
  }

  pub fn with_flags(expr: &str, flags: &str) -> Result<Self, Error> {
    let chars = flags.chars().collect::<Vec<char>>();
    Ok(Self {
      flags: chars.into_iter().collect(),
      source: expr.to_string(),
    })
  }

  pub fn global(&self) -> bool {
    self.flags.contains('g')
  }

  pub fn sticky(&self) -> bool {
    self.flags.contains('y')
  }
}
