#[deny(clippy::inherent_to_string)]
pub mod error;
pub mod locator;
pub mod result;
pub mod utils;

use crate::{
  error::{Error, FmsErrType},
  fms_sourcemap::{
    bit_set::BitSet,
    mappings::{MappingsFacade, SOURCE_INDEX},
    DecodedMap, SourceMap, SOURCEMAP_VERSION,
  },
  locator::Locator,
  result::Result,
  utils::{guess_indent, match_all, normalize_range, slice_string},
};

#[macro_use(concat_string)]
extern crate fms_utils;

use std::{cell::RefCell, collections::HashMap, rc::Rc, str, vec};

pub use fms_sourcemap;

pub mod chunk;
use chunk::Chunk;
use fms_utils::path::get_relative_path;
use regex::{Captures, Regex};

#[derive(Clone)]
pub struct GenerateMapOptions {
  pub file: Option<String>,
  pub source: Option<String>,
  pub source_root: Option<String>,
  pub include_content: Option<bool>,
  pub hires: Option<bool>,
}

impl Default for GenerateMapOptions {
  fn default() -> Self {
    Self {
      file: None,
      source_root: None,
      source: Some(String::default()),
      include_content: Some(false),
      hires: Some(false),
    }
  }
}

#[derive(Clone)]
pub struct OverwriteOptions {
  pub content_only: Option<bool>,
  pub store_name: Option<bool>,
  pub overwrite: Option<bool>,
}

impl Default for OverwriteOptions {
  fn default() -> Self {
    Self {
      content_only: Some(false),
      store_name: Some(false),
      overwrite: Some(false),
    }
  }
}

#[derive(Clone)]
pub enum IndentExclusionRanges {
  Single(Vec<u32>),
  Nested(Vec<Vec<u32>>),
}

#[derive(Clone)]
pub struct MagicStringOptions {
  pub filename: Option<String>,
  pub indent_exclusion_ranges: Option<IndentExclusionRanges>,
  pub ignore_list: Option<bool>,
}

impl Default for MagicStringOptions {
  fn default() -> Self {
    Self {
      filename: Some(String::default()),
      indent_exclusion_ranges: None,
      ignore_list: Some(false),
    }
  }
}

#[derive(Default)]
pub struct IndentOptions {
  pub exclude: Option<Vec<Vec<u32>>>,
  pub indent_start: Option<bool>,
}

#[allow(non_camel_case_types)]
pub struct MagicString {
  pub original: String,
  pub intro: String,
  pub outro: String,
  pub start_index_chunk_map: HashMap<u32, Rc<RefCell<Chunk>>>,
  pub end_index_chunk_map: HashMap<u32, Rc<RefCell<Chunk>>>,

  last_searched_chunk: Rc<RefCell<Chunk>>,
  first_chunk: Rc<RefCell<Chunk>>,
  last_chunk: Rc<RefCell<Chunk>>,
  stored_names: Vec<String>,
  ignore_list: bool,
  sourcemap_locations: BitSet,
  indent_str: Option<String>,
  _locator: Locator,
  _raw_options: MagicStringOptions,
}

impl MagicString {
  pub fn new(str: &str, options: Option<MagicStringOptions>) -> Self {
    let options = options.unwrap_or_default();
    let chunk = Rc::new(RefCell::new(Chunk::new(
      0,
      str.len().try_into().unwrap(),
      str,
    )));
    let init_start_index_chunk_map: Vec<(u32, Rc<RefCell<Chunk>>)> = vec![(0, chunk.clone())];
    let init_end_index_chunk_map: Vec<(u32, Rc<RefCell<Chunk>>)> =
      vec![(str.len().try_into().unwrap(), chunk.clone())];
    Self {
      original: str.to_string(),
      intro: String::default(),
      outro: String::default(),
      start_index_chunk_map: init_start_index_chunk_map.into_iter().collect(),
      end_index_chunk_map: init_end_index_chunk_map.into_iter().collect(),

      last_searched_chunk: Rc::clone(&chunk),
      first_chunk: Rc::clone(&chunk),
      last_chunk: Rc::clone(&chunk),
      stored_names: vec![],
      ignore_list: options.ignore_list.unwrap_or_default(),
      sourcemap_locations: BitSet::new(None),
      indent_str: None,
      _locator: Locator::new(str),
      _raw_options: options,
    }
  }

  pub fn add_sourcemap_location(&mut self, index: u32) -> &Self {
    self.sourcemap_locations.add(index as usize);
    self
  }

  pub fn append(&mut self, str: &str) -> Result<&mut Self> {
    self.outro = concat_string!(self.outro, str);
    Ok(self)
  }

  pub fn append_left(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.end_index_chunk_map.get(&index) {
      let mut chunk = chunk.borrow_mut();
      chunk.append_left(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn append_right(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.start_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.append_right(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn _clone(&self) -> MagicString {
    let mut cloned = MagicString::new(self.original.as_str(), Some(self._raw_options.clone()));
    cloned.first_chunk = Rc::new(RefCell::new(self.first_chunk.borrow().self_clone()));
    cloned.last_chunk = Rc::clone(&cloned.first_chunk);
    cloned.last_searched_chunk = Rc::clone(&cloned.first_chunk);

    let mut original_chunk = Some(Rc::clone(&self.first_chunk));
    let mut cloned_chunk = Some(Rc::clone(&cloned.first_chunk));

    while let Some(o) = original_chunk {
      if let Some(c) = cloned_chunk {
        // update `cloned chunk`
        cloned
          .start_index_chunk_map
          .insert(c.borrow().start, Rc::clone(&c));
        cloned
          .end_index_chunk_map
          .insert(c.borrow().end, Rc::clone(&c));

        // update `next cloned chunk`
        let original_next = o.borrow().next.clone();
        let cloned_next = if original_next.is_some() {
          let mut cloned_next = original_next.unwrap().borrow().self_clone();
          cloned_next.previous = Some(c.clone());
          Some(Rc::new(RefCell::new(cloned_next)))
        } else {
          None
        };

        // connect `cloned chunk` and `next cloned chunk`
        cloned_chunk = if cloned_next.is_some() {
          c.borrow_mut().next = cloned_next.clone();
          cloned.last_chunk = Rc::clone(&cloned_next.clone().unwrap());
          cloned_next.clone()
        } else {
          None
        }
      }
      original_chunk = o.borrow().next.clone();
    }

    cloned.intro = self.intro.clone();
    cloned.outro = self.outro.clone();
    cloned.sourcemap_locations = self.sourcemap_locations.clone();
    cloned
  }

  pub fn generate_map(&self, options: Option<GenerateMapOptions>) -> Result<SourceMap> {
    let decoded_map = self.generate_decoded_map(options)?;
    Ok(SourceMap::from_decoded_map(decoded_map)?)
  }

  pub fn generate_decoded_map(&self, options: Option<GenerateMapOptions>) -> Result<DecodedMap> {
    let GenerateMapOptions {
      file,
      source,
      hires,
      include_content,
      source_root,
    } = options.unwrap_or_default();
    let hires = hires.unwrap_or_default();
    let mut facade = MappingsFacade::new(hires, &self.sourcemap_locations);
    facade.advance(self.intro.as_str());

    Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      let loc = self._locator.locate(chunk.borrow().start as usize);
      if let Some((o_line, o_column)) = loc {
        facade.add_mappings(
          self.original.as_str(),
          chunk.borrow().content.as_str(),
          chunk.borrow().intro.as_str(),
          chunk.borrow().outro.as_str(),
          (o_line as u32, o_column as u32),
          (chunk.borrow().start, chunk.borrow().end),
          chunk.borrow().is_edited(),
          self
            .stored_names
            .binary_search(&chunk.borrow().original)
            .unwrap_or(usize::MAX),
        );
      }
      Ok(false)
    })?;
    facade.advance(self.outro.as_str());

    Ok(DecodedMap {
      version: SOURCEMAP_VERSION,
      file: file
        .as_ref()
        .and_then(|x| x.split(&['/', '\\'][..]).last().map(String::from)),
      sources: vec![source
        .as_ref()
        .map(|x| get_relative_path(&file.unwrap_or_default(), x))
        .unwrap_or_default()],
      sources_content: include_content.and_then(|x| {
        if x {
          Some(vec![self.original.to_owned()])
        } else {
          None
        }
      }),
      source_root,
      names: self.stored_names.to_owned(),
      mappings: facade.get(),
      x_google_ignoreList: if self.ignore_list {
        Some(vec![SOURCE_INDEX])
      } else {
        None
      },
    })
  }

  pub fn _move(&mut self, start: i32, end: i32, index: u32) -> Result<&mut Self> {
    let (_start, _end) = normalize_range(self.original.as_str(), start, end)?;

    if index >= _start && index <= _end {
      return Err(Error::from_reason(
        FmsErrType::Range,
        "Cannot move a selection inside itself",
      ));
    }

    self._split(_start)?;
    self._split(_end)?;
    self._split(index)?;

    let first = self
      .start_index_chunk_map
      .get(&_start)
      .map(Rc::clone)
      .unwrap();
    let last = self.end_index_chunk_map.get(&_end).map(Rc::clone).unwrap();

    let old_left = first.borrow().clone().previous;
    let old_right = last.borrow().clone().next;

    let new_right = self.start_index_chunk_map.get(&index).map(Rc::clone);
    let new_left = match new_right.clone() {
      Some(l) => Rc::clone(&l).borrow().clone().previous,
      None => Some(Rc::clone(&self.last_chunk)),
    };

    let _old_left = old_left.clone();

    match old_left {
      Some(old_left) => {
        old_left.borrow_mut().next = old_right.clone();
      }
      None => self.first_chunk = old_right.clone().unwrap(),
    }

    match old_right {
      Some(old_right) => old_right.borrow_mut().previous = _old_left,
      None => self.last_chunk = _old_left.unwrap(),
    }

    match new_left {
      Some(new_left) => {
        new_left.borrow_mut().next = Some(Rc::clone(&first));
        first.borrow_mut().previous = Some(new_left);
      }
      None => {
        let first = Rc::clone(&first);
        self.first_chunk.borrow_mut().previous = Some(Rc::clone(&first));
        last.borrow_mut().next = Some(Rc::clone(&self.first_chunk));
        self.first_chunk = first;
      }
    }

    match new_right {
      Some(new_right) => {
        new_right.borrow_mut().previous = Some(Rc::clone(&last));
        last.borrow_mut().next = Some(new_right);
      }
      None => {
        self.last_chunk.borrow_mut().next = Some(Rc::clone(&last));
        first.borrow_mut().previous = Some(Rc::clone(&self.last_chunk));
        last.borrow_mut().next = None;
        self.last_chunk = Rc::clone(&last);
      }
    }

    Ok(self)
  }

  pub fn overwrite(
    &mut self,
    start: i32,
    end: i32,
    content: &str,
    options: Option<OverwriteOptions>,
  ) -> Result<&mut Self> {
    let mut option = options.unwrap_or_default();
    option.overwrite = Some(!option.content_only.unwrap_or_default());
    self.update(start, end, content, Some(option))
  }

  pub fn update(
    &mut self,
    start: i32,
    end: i32,
    content: &str,
    options: Option<OverwriteOptions>,
  ) -> Result<&mut Self> {
    let option = options.unwrap_or_default();
    let store_name = option.store_name.unwrap_or_default();
    let content_only = option.content_only.unwrap_or_default();

    let (_start, _end) = normalize_range(self.original.as_str(), start, end)?;

    if _start == _end {
      return Err(Error::from_reason(
        FmsErrType::Range,
        "Cannot overwrite a zero-length range – use appendLeft or prependRight instead",
      ));
    }

    self._split(_start)?;
    self._split(_end)?;

    if store_name {
      let original = self.original.clone();
      self
        .stored_names
        .push(slice_string(original, _start as usize, _end as usize));
    }

    let first = self.start_index_chunk_map.get(&_start);
    let last = self.end_index_chunk_map.get(&_end);

    if first.is_some() && last.is_some() {
      let first = Rc::clone(first.unwrap());
      first
        .borrow_mut()
        .edit(content, store_name, !option.overwrite.unwrap_or(false));
      let last = Rc::clone(last.unwrap());
      let mut cur = Some(first);
      while cur.is_some() && cur.clone().unwrap() != last {
        let c = cur.as_ref().unwrap();
        if c.borrow().next.as_ref() != self.start_index_chunk_map.get(&c.borrow().end) {
          return Err(Error::from_reason(
            FmsErrType::Overwrite,
            "Cannot overwrite across a split point",
          ));
        }
        let next = c.borrow().next.clone();
        next.clone().unwrap().borrow_mut().edit("", false, false);
        cur = next;
      }
    } else {
      let mut new_chunk = Chunk::new(_start, _end, "");
      new_chunk.edit(content, store_name, content_only);

      if let Some(_last) = last {
        new_chunk.previous = Some(Rc::clone(_last));
        _last.borrow_mut().next = Some(Rc::new(RefCell::new(new_chunk)))
      }
    }

    Ok(self)
  }

  pub fn prepend(&mut self, str: &str) -> Result<&mut Self> {
    self.intro = concat_string!(str, self.intro);
    Ok(self)
  }

  pub fn prepend_left(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.end_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.prepend_left(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn prepend_right(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.start_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.prepend_right(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn remove(&mut self, start: i32, end: i32) -> Result<&Self> {
    let (_start, _end) = normalize_range(self.original.as_str(), start, end)?;

    if _start == _end {
      return Ok(self);
    }

    self._split(_start)?;
    self._split(_end)?;

    let first = self.start_index_chunk_map.get(&_start);
    let last = self.end_index_chunk_map.get(&_end);
    if last.is_some() {
      if let Some(cur) = first.map(Rc::clone) {
        let _ = Chunk::each_next(cur, |chunk| {
          if chunk.borrow().end > _end {
            Ok(true)
          } else {
            chunk.borrow_mut().edit("", false, false);
            Ok(false)
          }
        });
      }
    }
    Ok(self)
  }

  pub fn reset(&mut self, start: i32, end: i32) -> Result<&Self> {
    let (_start, _end) = normalize_range(self.original.as_str(), start, end)?;
    if _start == _end {
      return Ok(self);
    }
    self._split(_start)?;
    self._split(_end)?;
    let mut first = self.start_index_chunk_map.get(&_start).map(Rc::clone);
    while let Some(c) = first {
      c.borrow_mut().reset();
      first = if _end > c.borrow().end {
        self
          .start_index_chunk_map
          .get(&c.borrow().end)
          .map(Rc::clone)
      } else {
        None
      }
    }
    Ok(self)
  }

  pub fn slice(&self, start: i32, end: i32) -> Result<String> {
    let (_start, _end) = normalize_range(self.original.as_str(), start, end)?;
    let mut s = String::new();
    let mut chunk = Some(Rc::clone(&self.first_chunk));
    while let Some(cur) = chunk.clone() {
      if cur.borrow().start > _start || cur.borrow().end <= _start {
        if cur.borrow().start < _end && cur.borrow().end >= _end {
          return Ok(s);
        }
        chunk = cur.borrow().next.as_ref().map(Rc::clone);
      } else {
        break;
      }
    }
    if let Some(cur) = chunk.clone() {
      if cur.borrow().edited && cur.borrow().start != _start {
        return Err(Error::from_reason(
          FmsErrType::Slice,
          format!(
            "Cannot use replaced character {} as slice start anchor.",
            _start
          )
          .as_str(),
        ));
      }
    }
    let mut loop_idx = 0;
    if let Some(cur) = chunk {
      Chunk::each_next(cur, |c| {
        if !c.borrow().intro.is_empty() && (loop_idx != 0 || c.borrow().start == _start) {
          s.push_str(&c.borrow().intro);
        }
        let contains_end = c.borrow().start < _end && c.borrow().end >= _end;
        if contains_end && c.borrow().edited && c.borrow().end != _end {
          return Err(Error::from_reason(
            FmsErrType::Slice,
            format!(
              "Cannot use replaced character {} as slice end anchor.",
              _end
            )
            .as_str(),
          ));
        }
        let slice_start = if loop_idx == 0 {
          _start - c.borrow().start
        } else {
          0
        };
        let slice_end = if contains_end {
          (c.borrow().content.len()) as u32 + _end - c.borrow().end
        } else {
          c.borrow().content.len() as u32
        };
        s.push_str(&c.borrow().content.as_str()[slice_start as usize..slice_end as usize]);

        if !c.borrow().outro.is_empty() && (!contains_end || c.borrow().end == _end) {
          s.push_str(&c.borrow().outro);
        }
        loop_idx += 1;
        Ok(contains_end)
      })?;
    }
    Ok(s)
  }

  pub fn snip(&mut self, start: i32, end: i32) -> Result<MagicString> {
    let mut cloned = self._clone();
    cloned.remove(0, start)?;
    cloned.remove(end, cloned.original.len() as i32)?;
    Ok(cloned)
  }

  pub fn indent(
    &mut self,
    indent_str: Option<String>,
    options: Option<IndentOptions>,
  ) -> Result<&Self> {
    let options = options.unwrap_or_default();
    let mut indent_str = indent_str.map(|s| s.to_string());

    if indent_str.is_none() {
      self._ensure_indent_str()?;
      indent_str = self.indent_str.clone();
    }

    if let Some(ref s) = indent_str {
      if s.is_empty() {
        return Ok(self);
      }
    }

    let indent_str = indent_str.unwrap();

    let exclusions = options.exclude;
    let mut is_excluded_map = HashMap::new();

    if let Some(exclusions) = exclusions {
      for exclusion in exclusions {
        let start = exclusion[0];
        let end = exclusion[1];
        for i in start..end {
          is_excluded_map.insert(i, true);
        }
      }
    }

    let mut should_indent_next_character = options.indent_start.unwrap_or(true);

    let regexp = Regex::new(r"(?m)^[^\r\n]").unwrap();

    self.intro = regexp
      .replace_all(&self.intro, |caps: &regex::Captures| {
        if should_indent_next_character {
          concat_string!(indent_str, &caps[0])
        } else {
          should_indent_next_character = true;
          caps[0].to_string()
        }
      })
      .into_owned();

    let mut char_index = 0;
    let _ = Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      let content = chunk.borrow().content.clone();
      if chunk.borrow().is_edited() {
        let is_excluded = is_excluded_map.get(&char_index).copied().unwrap_or(false);
        if !is_excluded {
          chunk.borrow_mut().content = regexp
            .replace_all(&content, |caps: &regex::Captures| {
              if should_indent_next_character {
                concat_string!(indent_str, &caps[0])
              } else {
                should_indent_next_character = true;
                caps[0].to_string()
              }
            })
            .into_owned();
          if !chunk.borrow().content.is_empty() {
            should_indent_next_character =
              chunk.borrow().content.chars().last().unwrap_or_default() == '\n'
          }
        }
      } else {
        char_index = chunk.borrow().start;
        while char_index < chunk.borrow().end {
          let is_excluded = is_excluded_map.get(&char_index).copied().unwrap_or(false);
          if !is_excluded {
            if let Some(char) = self.original.chars().nth(char_index as usize) {
              if char == '\n' {
                should_indent_next_character = true;
              } else if char != '\r' && should_indent_next_character {
                should_indent_next_character = false;
                if char_index == chunk.borrow().start {
                  chunk.borrow_mut().prepend_right(&indent_str);
                } else {
                  self._split(char_index)?;
                  if let Some(next) = &chunk.borrow().next {
                    next.borrow_mut().prepend_right(&indent_str);
                  }
                }
              }
            }
          }
          char_index += 1;
        }
      }
      Ok(false)
    });

    self.outro = regexp
      .replace_all(&self.outro, |caps: &regex::Captures| {
        if should_indent_next_character {
          concat_string!(indent_str, &caps[0])
        } else {
          should_indent_next_character = true;
          caps[0].to_string()
        }
      })
      .into_owned();

    Ok(self)
  }

  pub fn is_empty(&self) -> bool {
    self.to_string().trim().is_empty()
  }

  pub fn trim(&mut self, char_type: Option<&str>) {
    self.trim_start(char_type).trim_end(char_type);
  }

  pub fn trim_start_aborted(&mut self, char_type: Option<&str>) -> bool {
    let pat = "^".to_owned() + char_type.unwrap_or("\\s") + "+";
    let regexp = Regex::new(pat.as_str()).unwrap();
    self.intro = regexp.replace(&self.intro, "").to_string();
    if !self.intro.is_empty() {
      return true;
    }
    let mut cur = Some(Rc::clone(&self.first_chunk));

    while let Some(c) = cur {
      let mut _cur = c.borrow_mut();
      // let end = _cur.end;
      let aborted = _cur.trim_start(&regexp);
      if aborted {
        return true;
      }
      cur = _cur.next.clone();
    }

    false
  }

  pub fn trim_start(&mut self, char_type: Option<&str>) -> &mut Self {
    self.trim_start_aborted(char_type);
    self
  }

  pub fn trim_end_aborted(&mut self, char_type: Option<&str>) -> bool {
    let pat = char_type.unwrap_or("\\s").to_owned() + "+$";
    let regexp = Regex::new(pat.as_str()).unwrap();
    self.outro = regexp.replace(&self.outro, "").to_string();
    if !self.outro.is_empty() {
      return true;
    }
    let mut cur = Some(Rc::clone(&self.last_chunk));

    while let Some(c) = cur {
      let mut _cur = c.borrow_mut();
      // let end = _cur.end;
      let aborted = _cur.trim_end(&regexp);
      if aborted {
        return true;
      }
      cur = _cur.previous.as_ref().map(Rc::clone);
    }

    false
  }

  pub fn trim_end(&mut self, char_type: Option<&str>) -> &mut Self {
    self.trim_end_aborted(char_type);
    self
  }

  pub fn trim_lines(&mut self) -> &mut Self {
    self.trim(Some("[\\r\\n]"));
    self
  }

  pub fn has_changed(&self) -> bool {
    self.original != self.to_string()
  }

  pub fn replace_by_regexp(
    &mut self,
    search_value: &str,
    replacement: &str,
    global: bool,
  ) -> Result<&Self> {
    let this = self as *mut Self;

    let regexp = Regex::new(search_value).unwrap();
    let str = self.original.as_str();
    let matches = match_all(&regexp, str, global);

    let get_replacement = |match_item: &Captures| {
      let regexp = Regex::new(r"\$(\$|&|(\d+))").unwrap();
      regexp.replace(replacement, |caps: &Captures| {
        let matched = &caps[0];
        let i = &caps[1];
        match i {
          "$" => "$".to_string(),
          "&" => matched.to_string(),
          num_str => {
            if let Ok(num) = num_str.parse::<usize>() {
              if num < match_item.len() {
                match_item.get(num).unwrap().as_str().to_string()
              } else {
                format!("${}", i)
              }
            } else {
              format!("${}", i)
            }
          }
        }
      })
    };

    for (idx, caps) in matches.0.iter().enumerate() {
      let _replacement = get_replacement(caps);
      let offset = matches.1.get(idx).unwrap_or(&0);
      let start = (caps.get(0).unwrap().start() + *offset) as i32;
      let end = (caps.get(0).unwrap().end() + *offset) as i32;
      unsafe {
        (*this).overwrite(start, end, _replacement.into_owned().as_str(), None)?;
      }
    }
    Ok(self)
  }

  pub fn replace_by_string(&mut self, search_value: &str, replacement: &str) -> Result<&Self> {
    let start = self.original.find(search_value);

    if let Some(start) = start {
      self.overwrite(
        start as i32,
        (start + search_value.chars().count()) as i32,
        replacement,
        None,
      )?;
    }

    Ok(self)
  }

  pub fn replace_all_by_string(&mut self, search_value: &str, replacement: &str) -> Result<&Self> {
    let mut start = self.original.find(search_value);
    let mut offset: usize = 0;
    while let Some(_start) = start {
      let _start = _start + offset;
      offset = _start + search_value.len();
      self.overwrite(_start as i32, offset as i32, replacement, None)?;
      start = if offset <= self.original.len() {
        self.original[offset..].find(search_value)
      } else {
        None
      }
    }

    Ok(self)
  }

  fn _ensure_indent_str(&mut self) -> Result<()> {
    if self.indent_str.is_none() {
      self.indent_str = Some(guess_indent(&self.original)?);
    }
    Ok(())
  }

  fn _split(&mut self, index: u32) -> Result<()> {
    if self.start_index_chunk_map.contains_key(&index)
      || self.end_index_chunk_map.contains_key(&index)
    {
      return Ok(());
    }

    let chunk = self.last_searched_chunk.clone();

    let is_forward = index < chunk.borrow().start;

    let mut cur = Some(chunk);

    while let Some(c) = cur {
      if c.borrow().contains(index) {
        self._split_chunk(c, index)?;
        return Ok(());
      } else {
        cur = {
          if is_forward {
            self
              .end_index_chunk_map
              .get(&c.borrow().start)
              .map(Rc::clone)
          } else {
            self
              .start_index_chunk_map
              .get(&c.borrow().end)
              .map(Rc::clone)
          }
        }
      }
    }

    Ok(())
  }

  fn _split_chunk(&mut self, chunk: Rc<RefCell<Chunk>>, index: u32) -> Result<()> {
    if chunk.borrow().is_edited() && !chunk.borrow().content.is_empty() {
      if let Some((line, column)) = self._locator.locate(index as usize) {
        return Err(Error::from_reason(
          FmsErrType::SplitChunk,
          format!(
            "Cannot split a chunk that has already been edited ({}:{} – '{}')",
            line,
            column,
            chunk.borrow().original
          )
          .as_str(),
        ));
      } else {
        return Err(Error::from_reason(
          FmsErrType::SplitChunk,
          "Cannot split a chunk that has already been edited",
        ));
      }
    }

    let new_chunk = Chunk::split(Rc::clone(&chunk), index)?;
    self
      .start_index_chunk_map
      .insert(index, Rc::clone(&new_chunk));
    self.end_index_chunk_map.insert(index, Rc::clone(&chunk));
    self
      .end_index_chunk_map
      .insert(new_chunk.borrow().end, Rc::clone(&new_chunk));
    self.last_searched_chunk = Rc::clone(&chunk);
    if self.last_chunk == chunk {
      self.last_chunk = new_chunk
    }
    Ok(())
  }

  pub fn to_string(&self) -> String {
    let mut str = self.intro.clone();
    let _ = Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      str.push_str(chunk.borrow().intro.as_str());
      str.push_str(chunk.borrow().content.as_str());
      str.push_str(chunk.borrow().outro.as_str());
      Ok(false)
    });
    str.push_str(self.outro.as_str());
    str
  }
}
