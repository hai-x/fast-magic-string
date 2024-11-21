use fast_magic_string::{
  fms_sourcemap::{DecodedMap, SourceMap},
  GenerateMapOptions, IndentExclusionRanges, IndentOptions, MagicStringOptions, OverwriteOptions,
};
use napi::Either;

#[napi(object)]
pub struct JsIndentOptions {
  pub exclude: Option<Vec<Vec<u32>>>,
  pub indent_start: Option<bool>,
}

impl From<JsIndentOptions> for IndentOptions {
  fn from(js_indent_options: JsIndentOptions) -> Self {
    IndentOptions {
      exclude: js_indent_options.exclude,
      indent_start: js_indent_options.indent_start,
    }
  }
}

#[napi(object)]
#[derive(Clone)]
pub struct JsMagicStringOptions {
  pub filename: Option<String>,
  pub indent_exclusion_ranges: Option<Either<Vec<u32>, Vec<Vec<u32>>>>,
  pub ignore_list: Option<bool>,
}

impl From<JsMagicStringOptions> for MagicStringOptions {
  fn from(js_magic_string_options: JsMagicStringOptions) -> Self {
    let indent_exclusion_ranges =
      if let Some(indent_exclusion_ranges) = js_magic_string_options.indent_exclusion_ranges {
        match indent_exclusion_ranges {
          Either::A(v) => Some(IndentExclusionRanges::Single(v)),
          Either::B(v) => Some(IndentExclusionRanges::Nested(v)),
        }
      } else {
        None
      };
    MagicStringOptions {
      filename: js_magic_string_options.filename,
      indent_exclusion_ranges,
      ignore_list: js_magic_string_options.ignore_list,
    }
  }
}

#[napi(object)]
pub struct JsGenerateMapOptions {
  pub file: Option<String>,
  pub source: Option<String>,
  pub source_root: Option<String>,
  pub include_content: Option<bool>,
  pub hires: Option<bool>,
}

impl From<JsGenerateMapOptions> for GenerateMapOptions {
  fn from(js_generate_map_options: JsGenerateMapOptions) -> Self {
    GenerateMapOptions {
      file: js_generate_map_options.file,
      source: js_generate_map_options.source,
      source_root: js_generate_map_options.source_root,
      include_content: js_generate_map_options.include_content,
      hires: js_generate_map_options.hires,
    }
  }
}

#[derive(Clone)]
#[napi(object)]
pub struct JsOverwriteOptions {
  pub content_only: Option<bool>,
  pub store_name: Option<bool>,
  pub overwrite: Option<bool>,
}

impl From<JsOverwriteOptions> for OverwriteOptions {
  fn from(js_overwrite_options: JsOverwriteOptions) -> Self {
    OverwriteOptions {
      content_only: js_overwrite_options.content_only,
      store_name: js_overwrite_options.store_name,
      overwrite: js_overwrite_options.overwrite,
    }
  }
}

#[napi(object)]
pub struct FmsRegex {
  pub global: Option<bool>,
  pub rule: String,
}

#[napi(object)]
pub struct JsSourceMap {
  pub version: u8,
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub sources: Vec<String>,
  pub sources_content: Option<Vec<String>>,
  pub names: Vec<String>,
  pub mappings: String,
  pub x_google_ignoreList: Option<Vec<u8>>,
}

impl From<SourceMap> for JsSourceMap {
  fn from(source_map: SourceMap) -> Self {
    JsSourceMap {
      version: source_map.version,
      file: source_map.file,
      source_root: source_map.source_root,
      sources: source_map.sources,
      sources_content: source_map.sources_content,
      names: source_map.names,
      mappings: source_map.mappings,
      x_google_ignoreList: source_map.x_google_ignoreList,
    }
  }
}

#[napi(object)]
pub struct JsDecodedMap {
  pub version: u8,
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub sources: Vec<String>,
  pub sources_content: Option<Vec<String>>,
  pub names: Vec<String>,
  pub mappings: Vec<Vec<Vec<i64>>>,
  pub x_google_ignoreList: Option<Vec<u8>>,
}

impl From<DecodedMap> for JsDecodedMap {
  fn from(decoded_map: DecodedMap) -> Self {
    JsDecodedMap {
      version: decoded_map.version,
      file: decoded_map.file,
      source_root: decoded_map.source_root,
      sources: decoded_map.sources,
      sources_content: decoded_map.sources_content,
      names: decoded_map.names,
      mappings: decoded_map.mappings,
      x_google_ignoreList: decoded_map.x_google_ignoreList,
    }
  }
}
