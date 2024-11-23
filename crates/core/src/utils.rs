use crate::error::{Error, MsErrType};
use crate::result::Result;

pub fn slice_string(s: String, start: usize, end: usize) -> String {
  s[start..end].to_owned()
}

pub fn _normalize_range(str: &str, start: i32, end: i32) -> Result<(u32, u32)> {
  let mut _start = start;
  let mut _end = end;
  let len = str.len() as i32;
  if len > 0 {
    while _start < 0 {
      _start += len;
    }
    while _end < 0 {
      _end += len;
    }
  }

  if _end > len {
    return Err(Error::from_reason(MsErrType::Range, "end is out of bounds"));
  }
  Ok((_start as u32, _end as u32))
}

use regex::{Captures, Regex};

pub fn match_all<'a>(re: &Regex, text: &'a str, global: bool) -> (Vec<Captures<'a>>, Vec<usize>) {
  let mut matches = Vec::new();
  let mut start = 0;
  let mut offsets = Vec::new();

  while let Some(captures) = re.captures(&text[start..]) {
    offsets.push(start);
    start += captures.get(0).unwrap().end();
    matches.push(captures);
    if !global {
      return (matches, offsets);
    }
  }
  (matches, offsets)
}

pub fn get_relative_path(from: &str, to: &str) -> String {
  let from_parts: Vec<&str> = from.split(&['/', '\\'][..]).collect();
  let to_parts: Vec<&str> = to.split(&['/', '\\'][..]).collect();

  let mut from_parts_clone = from_parts.clone();
  from_parts_clone.pop();

  let mut common_length = 0;
  while common_length < from_parts_clone.len() && common_length < to_parts.len() {
    if from_parts_clone[common_length] == to_parts[common_length] {
      common_length += 1;
    } else {
      break;
    }
  }

  let mut relative_path = Vec::new();

  for _ in common_length..from_parts_clone.len() {
    relative_path.push("..");
  }

  for part in &to_parts[common_length..] {
    relative_path.push(part);
  }

  relative_path.join("/")
}

pub fn guess_indent(code: &str) -> Result<String> {
  let lines: Vec<&str> = code.lines().collect();

  let tabbed: Vec<&str> = lines
    .iter()
    .filter(|line| line.starts_with('\t'))
    .cloned()
    .collect();
  let spaced: Vec<&str> = lines
    .iter()
    .filter(|line| line.starts_with("  "))
    .cloned()
    .collect();

  if tabbed.len() >= spaced.len() || (tabbed.is_empty() && spaced.is_empty()) {
    return Ok("\t".to_string());
  }

  let min_spaces = spaced.iter().fold(usize::MAX, |min_spaces, line| {
    let num_spaces = line.chars().take_while(|&c| c == ' ').count();
    min_spaces.min(num_spaces)
  });

  Ok(" ".repeat(min_spaces))
}

pub fn find_char_index_of_substring(s: &str, substring: &str) -> Option<usize> {
  let mut char_index = 0;
  let mut byte_index = 0;
  let substring_length = substring.chars().count();

  for (_, ch) in s.chars().enumerate() {
    if s[byte_index..]
      .chars()
      .take(substring_length)
      .collect::<String>()
      == substring
    {
      return Some(char_index);
    }
    char_index += 1;
    byte_index += ch.len_utf8()
  }

  None
}
