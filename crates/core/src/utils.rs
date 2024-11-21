use crate::error::{Error, FmsErrType};
use crate::result::Result;

pub fn slice_string(s: String, start: usize, end: usize) -> String {
  s[start..end].to_owned()
}

pub fn normalize_range(str: &str, start: i32, end: i32) -> Result<(u32, u32)> {
  let mut _start = start;
  let mut _end = end;
  let len = str.len().try_into().unwrap();
  if len > 0 {
    while _start < 0 {
      _start += len;
    }
    while _end < 0 {
      _end += len;
    }
  }

  if _end > len {
    return Err(Error::from_reason(
      FmsErrType::Range,
      "end is out of bounds",
    ));
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
