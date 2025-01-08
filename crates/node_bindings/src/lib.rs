extern crate fms_malloc;
extern crate napi;

#[macro_use]
extern crate napi_derive;

use error::to_napi_error;
use fms_regexp::FmsRegex;
use napi::{
  bindgen_prelude::{Either, Function},
  Result,
};

extern crate fast_magic_string;

use fast_magic_string::{
  error::{Error, FmsErrType},
  MagicString,
};
mod types;

use types::{
  JsDecodedMap, JsGenerateMapOptions, JsIndentOptions, JsMagicStringOptions, JsOverwriteOptions,
  JsSourceMap,
};

mod error;

#[napi(js_name = "MagicString")]
struct JsMagicString {
  inner: MagicString,
  pub indent_exclusion_ranges: Option<Either<Vec<u32>, Vec<Vec<u32>>>>,
}

#[allow(dead_code)]
#[napi]
impl JsMagicString {
  #[napi(constructor)]
  pub fn new(str: String, options: Option<JsMagicStringOptions>) -> JsMagicString {
    JsMagicString {
      indent_exclusion_ranges: options
        .as_ref()
        .and_then(|o| o.indent_exclusion_ranges.clone()),
      inner: MagicString::new(str.as_str(), options.map(|x| x.into())),
    }
  }

  #[napi]
  pub fn add_sourcemap_location(&mut self, index: u32) -> Result<&Self> {
    self.inner.add_sourcemap_location(index);
    Ok(self)
  }

  #[napi]
  pub fn append(&mut self, input: String) -> Result<&Self> {
    self.inner.append(input.as_str()).map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn append_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self
      .inner
      .append_left(index, input.as_str())
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn clone(&self) -> JsMagicString {
    let inner = self.inner._clone();
    let indent_exclusion_ranges = self.indent_exclusion_ranges.clone();
    JsMagicString {
      indent_exclusion_ranges,
      inner,
    }
  }

  #[napi]
  pub fn generate_map(&mut self, options: Option<JsGenerateMapOptions>) -> Result<JsSourceMap> {
    let map = self
      .inner
      .generate_map(options.map(|x| x.into()))
      .map_err(to_napi_error)?
      .into();
    Ok(map)
  }

  #[napi]
  pub fn generate_decoded_map(
    &mut self,
    options: Option<JsGenerateMapOptions>,
  ) -> Result<JsDecodedMap> {
    let decoded_map = self
      .inner
      .generate_decoded_map(options.map(|x| x.into()))
      .map_err(to_napi_error)?
      .into();
    Ok(decoded_map)
  }

  #[napi]
  pub fn indent(
    &mut self,
    indent_str: Option<String>,
    options: Option<JsIndentOptions>,
  ) -> Result<&Self> {
    self
      .inner
      .indent(indent_str, options.map(|x| x.into()))
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn insert(&mut self) -> Result<()> {
    Err(to_napi_error(Error::from_reason(
      FmsErrType::Deprecated,
      "magicString.insert(...) is deprecated. Use prependRight(...) or appendLeft(...)",
    )))
  }

  #[napi]
  pub fn insert_left(&mut self, index: u32, input: String) -> Result<&Self> {
    println!("magicString.insertLeft(...) is deprecated. Use magicString.appendLeft(...) instead");
    self
      .inner
      .append_left(index, input.as_str())
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn append_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self
      .inner
      .append_right(index, input.as_str())
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn prepend(&mut self, input: String) -> Result<&Self> {
    self.inner.prepend(input.as_str()).map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self
      .inner
      .prepend_left(index, input.as_str())
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self
      .inner
      .prepend_right(index, input.as_str())
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn insert_right(&mut self, index: u32, input: String) -> Result<&Self> {
    println!(
      "magicString.insertRight(...) is deprecated. Use magicString.prependRight(...) instead"
    );
    self
      .inner
      .prepend_right(index, input.as_str())
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn trim(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.inner.trim(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_lines(&mut self) -> Result<&Self> {
    self.inner.trim_lines();
    Ok(self)
  }

  #[napi]
  pub fn trim_start(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.inner.trim_start(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_end(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.inner.trim_end(char_type.as_deref());
    Ok(self)
  }

  #[napi(js_name = "move")]
  pub fn _move(&mut self, start: i32, end: i32, index: u32) -> Result<&Self> {
    self.inner._move(start, end, index).map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn remove(&mut self, start: i32, end: i32) -> Result<&Self> {
    self.inner.remove(start, end).map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn overwrite(
    &mut self,
    start: i32,
    end: i32,
    content: String,
    options: Option<JsOverwriteOptions>,
  ) -> Result<&Self> {
    self
      .inner
      .overwrite(start, end, content.as_str(), options.map(|x| x.into()))
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn update(
    &mut self,
    start: i32,
    end: i32,
    content: String,
    options: Option<JsOverwriteOptions>,
  ) -> Result<&Self> {
    self
      .inner
      .update(start, end, content.as_str(), options.map(|x| x.into()))
      .map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  pub fn to_string(&mut self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn has_changed(&self) -> bool {
    self.inner.has_changed()
  }

  #[napi]
  pub fn snip(&mut self, start: i32, end: i32) -> Result<JsMagicString> {
    let inner = self.inner.snip(start, end).map_err(to_napi_error)?;
    let indent_exclusion_ranges = self.indent_exclusion_ranges.clone();
    Ok(JsMagicString {
      indent_exclusion_ranges,
      inner,
    })
  }

  #[napi]
  pub fn slice(&mut self, start: Option<i32>, end: Option<i32>) -> Result<String> {
    let _start = start.unwrap_or(0);
    let _end = end.unwrap_or(self.inner.original.len().try_into().unwrap());
    Ok(self.inner.slice(_start, _end).map_err(to_napi_error)?)
  }

  #[napi]
  pub fn reset(&mut self, start: i32, end: i32) -> Result<&Self> {
    self.inner.reset(start, end).map_err(to_napi_error)?;
    Ok(self)
  }

  #[napi]
  pub fn replace(
    &mut self,
    #[napi(ts_arg_type = "RegExp | string")] pattern: Either<String, FmsRegex>,
    replacement: Either<String, Function>,
  ) -> Result<&Self> {
    match replacement {
      Either::A(replacement) => match pattern {
        Either::A(str) => {
          self
            .inner
            .replace_by_string(str.as_str(), replacement.as_str())
            .map_err(to_napi_error)?;
        }
        Either::B(reg) => {
          self
            .inner
            .replace_by_regexp(reg.source.as_str(), replacement.as_str(), reg.global())
            .map_err(to_napi_error)?;
        }
      },
      Either::B(_) => {
        return Err(to_napi_error(Error::from_reason(
          FmsErrType::Type,
          "`replacement` argument do not supports RegExp replacerFn now",
        )));
      }
    }

    Ok(self)
  }

  #[napi]
  pub fn replace_all(
    &mut self,
    #[napi(ts_arg_type = "RegExp | string")] pattern: Either<String, FmsRegex>,
    replacement: Either<String, Function>,
  ) -> Result<&Self> {
    match replacement {
      Either::A(replacement) => match pattern {
        Either::A(pattern) => {
          self
            .inner
            .replace_all_by_string(pattern.as_str(), replacement.as_str())
            .map_err(to_napi_error)?;
        }
        Either::B(reg) => {
          let global = reg.global();
          // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll#pattern
          // > If pattern is a regex, then it must have the global (g) flag set, or a TypeError is thrown.
          if !global {
            return Err(to_napi_error(Error::from_reason(
              FmsErrType::Type,
              "replaceAll called with a non-global RegExp argument",
            )));
          }
          self
            .inner
            .replace_by_regexp(reg.source.as_str(), replacement.as_str(), global)
            .map_err(to_napi_error)?;
        }
      },
      Either::B(_) => {
        return Err(to_napi_error(Error::from_reason(
          FmsErrType::Type,
          "`replacement` argument do not supports RegExp replacerFn now",
        )));
      }
    }

    Ok(self)
  }
}
