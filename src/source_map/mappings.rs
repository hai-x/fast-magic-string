/**
 * Most of the implementations refer to
 * https://github.com/h-a-n-a/magic-string-rs/blob/main/core/src/mapping.rs
 */
use std::{cell::RefCell, rc::Rc};

use crate::internal_magic_string::chunk::Chunk;
use crate::result::Result;

pub type Seg = Vec<i64>;
pub type Line = Vec<Seg>;

pub type Mappings = Vec<Line>;

pub static SOURCE_INDEX: u8 = 0;

#[derive(Debug)]
pub struct MappingsFacade {
  pub raw: Mappings,
  generated_code_line: u32,
  generated_code_column: u32,
  hires: bool,
}

impl MappingsFacade {
  pub fn new(hires: bool) -> Self {
    Self {
      generated_code_line: 0,
      generated_code_column: 0,
      hires,
      raw: vec![],
    }
  }

  pub fn add_chunk(
    &mut self,
    chunk: Rc<RefCell<Chunk>>,
    (o_line, o_column): (u32, u32),
    name_index: usize,
  ) {
    let is_edited = chunk.borrow().is_edited();

    if !chunk.borrow().intro.is_empty() {
      self.advance(chunk.borrow().intro.as_str());
    }
    if is_edited {
      let content = chunk.borrow().content.clone();
      let lines: Vec<&str> = content.split('\n').collect();
      let lines_len = lines.len();

      for (index, &s) in lines.iter().enumerate() {
        if !s.is_empty() {
          let mut seg: Seg = vec![
            self.generated_code_column.into(),
            SOURCE_INDEX.into(),
            o_line.into(),
            o_column.into(),
          ];

          if name_index < usize::MAX {
            seg.push(name_index as i64);
          }
          if let Some(line) = self.raw.get_mut(self.generated_code_line as usize) {
            line.push(seg);
          } else {
            self.raw.push(vec![seg]);
          }
        }
        if index != lines_len - 1 {
          self.generated_code_column = 0;
          self.generated_code_line += 1;
        } else {
          self.generated_code_column += s.len() as u32;
        }
      }
    } else {
      let mut o_line = o_line;
      let mut o_column = o_column;
      let mut first = true;
      for char in chunk.borrow().original.chars() {
        if self.hires || first {
          let seg: Seg = vec![
            self.generated_code_column.into(),
            SOURCE_INDEX.into(),
            o_line.into(),
            o_column.into(),
          ];

          if let Some(line) = self.raw.get_mut(self.generated_code_line as usize) {
            line.push(seg);
          } else {
            self.raw.push(vec![seg]);
          }
        }
        match char {
          '\n' => {
            o_line += 1;
            o_column = 0;
            self.generated_code_line += 1;
            self.generated_code_column = 0;
            first = true
          }
          _ => {
            o_column += 1;
            self.generated_code_column += 1;
            first = false
          }
        }
      }
    }
    if !chunk.borrow().outro.is_empty() {
      self.advance(chunk.borrow().outro.as_str());
    }
  }

  pub fn advance(&mut self, str: &str) {
    if str.is_empty() {
      return;
    }
    let lines: Vec<&str> = str.split("\n").collect();

    if lines.len() > 1 {
      for _ in 0..lines.len() - 1 {
        self.generated_code_line += 1;
        self.raw.push(Vec::default());
      }
      self.generated_code_column = 0;
    }

    self.generated_code_column += lines.last().unwrap().len() as u32;
  }

  pub fn get_decoded_mappings(&mut self) -> Mappings {
    let mut source_index: i64 = 0;
    let mut original_line: i64 = 0;
    let mut original_column: i64 = 0;

    let decoded_mappings = self
      .raw
      .iter()
      .map(|line| {
        let mut generated_column: i64 = 0;

        line
          .iter()
          .map(|segment| {
            let generated_column_offset = segment[0] - generated_column;
            let source_index_offset = segment[1] - source_index;
            let original_line_offset = segment[2] - original_line;
            let original_column_offset = segment[3] - original_column;

            generated_column = segment[0];
            source_index = segment[1];
            original_line = segment[2];
            original_column = segment[3];

            vec![
              generated_column_offset,
              source_index_offset,
              original_line_offset,
              original_column_offset,
            ]
          })
          .collect::<Line>()
      })
      .collect::<Mappings>();

    decoded_mappings
  }
}

pub fn serialize_mappings(raw_mappings: &Mappings) -> Result<String> {
  let mut res: Vec<String> = vec![];
  for line in raw_mappings.iter() {
    let mut line_str: Vec<String> = vec![];
    for seg in line.iter() {
      let mut seg_str: Vec<String> = vec![];
      for item in seg.iter() {
        let mut vlq_output: Vec<u8> = vec![];
        // vlq need i64
        vlq::encode(item.to_owned(), &mut vlq_output)?;
        seg_str.push(String::from_utf8(vlq_output)?);
      }
      line_str.push(seg_str.join(""));
    }
    res.push(line_str.join(","));
  }
  Ok(res.join(";"))
}