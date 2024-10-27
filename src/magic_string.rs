use crate::{
  error::{Error, MsErrType},
  internal_magic_string::{
    GenerateMapOptions, IndentOptions, MagicStringOptions, OverwriteOptions,
    __internal_magic_string,
  },
  source_map::{DecodedMap, SourceMap},
  utils::JsRegExp,
};

use napi::{bindgen_prelude::Either, JsFunction, Result};

#[napi]
struct MagicString(__internal_magic_string);

#[allow(dead_code)]
#[napi]
impl MagicString {
  #[napi(constructor)]
  pub fn new(str: String, options: Option<MagicStringOptions>) -> MagicString {
    MagicString(__internal_magic_string::new(str.as_str(), options))
  }

  #[napi]
  pub fn add_sourcemap_location(&mut self, index: u32) -> Result<&Self> {
    self.0.add_sourcemap_location(index);
    Ok(self)
  }

  #[napi]
  pub fn append(&mut self, input: String) -> Result<&Self> {
    self.0.append(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn clone(&self) -> MagicString {
    let ms = self.0.clone();
    MagicString(ms)
  }

  #[napi]
  pub fn generate_map(&mut self, options: Option<GenerateMapOptions>) -> Result<SourceMap> {
    let map = self.0.generate_map(options)?;
    Ok(map)
  }

  #[napi]
  pub fn generate_decoded_map(
    &mut self,
    options: Option<GenerateMapOptions>,
  ) -> Result<DecodedMap> {
    let map = self.0.generate_decoded_map(options)?;
    Ok(map)
  }

  #[napi]
  pub fn indent(
    &mut self,
    indent_str: Option<String>,
    options: Option<IndentOptions>,
  ) -> Result<&Self> {
    self.0.indent(indent_str, options)?;
    Ok(self)
  }

  #[napi]
  pub fn insert(&mut self) -> Result<()> {
    Err(
      Error::from_reason(
        MsErrType::Deprecated,
        "magicString.insert(...) is deprecated. Use prependRight(...) or appendLeft(...)",
      )
      .into(),
    )
  }

  #[napi]
  pub fn insert_left(&mut self, index: u32, input: String) -> Result<&Self> {
    println!("magicString.insertLeft(...) is deprecated. Use magicString.appendLeft(...) instead");
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.append_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend(&mut self, input: String) -> Result<&Self> {
    self.0.prepend(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.prepend_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn insert_right(&mut self, index: u32, input: String) -> Result<&Self> {
    println!(
      "magicString.insertRight(...) is deprecated. Use magicString.prependRight(...) instead"
    );
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn trim(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.0.trim(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_lines(&mut self) -> Result<&Self> {
    self.0.trim_lines();
    Ok(self)
  }

  #[napi]
  pub fn trim_start(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.0.trim_start(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_end(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.0.trim_end(char_type.as_deref());
    Ok(self)
  }

  #[napi(js_name = "move")]
  pub fn _move(&mut self, start: i32, end: i32, index: u32) -> Result<&Self> {
    self.0._move(start, end, index)?;
    Ok(self)
  }

  #[napi]
  pub fn remove(&mut self, start: i32, end: i32) -> Result<&Self> {
    self.0.remove(start, end)?;
    Ok(self)
  }

  #[napi(ts_args_type = r"
    start: number,
    end: number,
    content: string,
    options?: OverwriteOptions
  ")]
  pub fn overwrite(
    &mut self,
    start: i32,
    end: i32,
    content: String,
    options: Option<OverwriteOptions>,
  ) -> Result<&Self> {
    self.0.overwrite(start, end, content.as_str(), options)?;
    Ok(self)
  }

  #[napi(ts_args_type = r"
    start: number,
    end: number,
    content: string,
    options?: OverwriteOptions
  ")]
  pub fn update(
    &mut self,
    start: i32,
    end: i32,
    content: String,
    options: Option<OverwriteOptions>,
  ) -> Result<&Self> {
    self.0.update(start, end, content.as_str(), options)?;
    Ok(self)
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[napi]
  pub fn to_string(&mut self) -> String {
    self.0.to_string()
  }

  #[napi]
  pub fn has_changed(&self) -> bool {
    self.0.has_changed()
  }

  #[napi]
  pub fn snip(&mut self, start: i32, end: i32) -> Result<MagicString> {
    let ms = self.0.snip(start, end)?;
    Ok(MagicString(ms))
  }

  #[napi]
  pub fn slice(&mut self, start: Option<i32>, end: Option<i32>) -> Result<String> {
    let _start = start.unwrap_or(0);
    let _end = end.unwrap_or(self.0.original.len() as i32);
    let ms = self.0.slice(_start, _end)?;
    Ok(ms)
  }

  #[napi]
  pub fn reset(&mut self, start: i32, end: i32) -> Result<&Self> {
    self.0.reset(start, end)?;
    Ok(self)
  }

  #[napi]
  pub fn replace(
    &mut self,
    search_value: Either<String, JsRegExp>,
    replacement: Either<String, JsFunction>,
  ) -> Result<&Self> {
    match replacement {
      Either::A(replacement) => match search_value {
        Either::A(str) => {
          self.0._replace_string(str.as_str(), replacement.as_str())?;
        }
        Either::B(reg_exp) => {
          self.0._replace_regexp(
            reg_exp.rule.as_str(),
            replacement.as_str(),
            reg_exp.global.unwrap_or_default(),
          )?;
        }
      },
      Either::B(_) => {
        return Err(
          Error::from_reason(
            MsErrType::Type,
            "`replacement` argument do not supports RegExp replacerFn now",
          )
          .into(),
        );
      }
    }

    Ok(self)
  }

  #[napi]
  pub fn replace_all(
    &mut self,
    search_value: Either<String, JsRegExp>,
    replacement: Either<String, JsFunction>,
  ) -> Result<&Self> {
    match replacement {
      Either::A(replacement) => match search_value {
        Either::A(search_value) => {
          self
            .0
            ._replace_all_string(search_value.as_str(), replacement.as_str())?;
        }
        Either::B(reg_exp) => {
          let global = reg_exp.global.unwrap_or_default();
          if !global {
            return Err(
              Error::from_reason(
                MsErrType::Type,
                "replaceAll called with a non-global RegExp argument",
              )
              .into(),
            );
          }
          self
            .0
            ._replace_regexp(reg_exp.rule.as_str(), replacement.as_str(), global)?;
        }
      },
      Either::B(_) => {
        return Err(
          Error::from_reason(
            MsErrType::Type,
            "`replacement` argument do not supports RegExp replacerFn now",
          )
          .into(),
        );
      }
    }

    Ok(self)
  }
}
