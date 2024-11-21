use crate::bit_set::BitSet;
use crate::error::SourcemapError;

pub type Seg = Vec<i64>;
pub type Line = Vec<Seg>;

pub type Mappings = Vec<Line>;

pub static SOURCE_INDEX: u8 = 0;

pub struct MappingsFacade {
  pub raw: Mappings,
  generated_code_line: u32,
  generated_code_column: u32,
  hires: bool,
  sourcemap_locations: BitSet,
}

impl MappingsFacade {
  pub fn new(hires: bool, sourcemap_locations: &BitSet) -> Self {
    Self {
      generated_code_line: 0,
      generated_code_column: 0,
      hires,
      raw: vec![],
      sourcemap_locations: BitSet::new(Some(sourcemap_locations)),
    }
  }

  pub fn add_mappings(
    &mut self,
    string_original: &str,
    chunk_content: &str,
    chunk_intro: &str,
    chunk_outro: &str,
    (origin_line, origin_column): (u32, u32),
    (chunk_start, chunk_end): (u32, u32),
    chunk_is_edited: bool,
    name_index: usize,
  ) {
    if !chunk_intro.is_empty() {
      self.advance(chunk_intro);
    }
    if chunk_is_edited {
      let lines: Vec<&str> = chunk_content.split('\n').collect();
      let lines_len = lines.len();

      for (index, &s) in lines.iter().enumerate() {
        if !s.is_empty() {
          let mut seg: Seg = vec![
            self.generated_code_column.into(),
            SOURCE_INDEX.into(),
            origin_line.into(),
            origin_column.into(),
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
      let mut original_char_index = chunk_start as usize;
      let mut o_line = origin_line;
      let mut o_column = origin_column;
      let mut first = true;

      while original_char_index < chunk_end as usize {
        if self.hires || first || self.sourcemap_locations.has(original_char_index) {
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
        match string_original.chars().nth(original_char_index).unwrap() {
          '\n' => {
            o_line += 1;
            self.generated_code_line += 1;
            o_column = 0;
            self.generated_code_column = 0;
            first = true
          }
          _ => {
            o_column += 1;
            self.generated_code_column += 1;
            first = false
          }
        }

        original_char_index += 1;
      }
    }
    if !chunk_outro.is_empty() {
      self.advance(chunk_outro);
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

  pub fn get(&mut self) -> Mappings {
    let mut source_index: i64 = 0;
    let mut original_line: i64 = 0;
    let mut original_column: i64 = 0;

    self
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
      .collect::<Mappings>()
  }
}

pub fn encode_mappings(raw_mappings: &Mappings) -> Result<String, SourcemapError> {
  // see https://github.com/hoodie/concatenation_benchmarks-rs
  let mut s = String::new();
  for (line_idx, line) in raw_mappings.iter().enumerate() {
    let mut line_s = String::new();
    for (seg_idx, seg) in line.iter().enumerate() {
      let mut seg_s = String::new();
      for item in seg.iter() {
        let mut vlq_output: Vec<u8> = vec![];
        // vlq need i64
        vlq::encode(item.to_owned(), &mut vlq_output)?;
        seg_s.push_str(&String::from_utf8(vlq_output)?);
      }
      line_s.push_str(&seg_s);
      if seg_idx != line.len() - 1 {
        line_s.push_str(",");
      }
    }
    s.push_str(&line_s);
    if line_idx != raw_mappings.len() - 1 {
      s.push_str(";");
    }
  }
  Ok(s)
}
