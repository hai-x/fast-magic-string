use std::{cell::RefCell, rc::Rc};

use regex::Regex;

use crate::error::{Error, FmsErrType};
use crate::result::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
  pub start: u32,
  pub end: u32,
  pub original: String,
  pub intro: String,
  pub outro: String,
  pub content: String,
  pub store_name: bool,
  pub edited: bool,
  pub previous: Option<Rc<RefCell<Chunk>>>,
  pub next: Option<Rc<RefCell<Chunk>>>,
}

impl Chunk {
  pub fn new(start: u32, end: u32, content: &str) -> Self {
    Chunk {
      start,
      end,
      original: content.to_string(),
      intro: String::default(),
      outro: String::default(),
      content: content.to_string(),
      store_name: false,
      edited: false,
      previous: None,
      next: None,
    }
  }

  pub fn is_edited(&self) -> bool {
    self.edited || (self.original.len() != self.content.len()) || self.original != self.content
  }

  pub fn self_clone(&self) -> Chunk {
    let mut cloned = Chunk::new(self.start, self.end, self.original.as_str());
    cloned.intro = self.intro.clone();
    cloned.intro = self.outro.clone();
    cloned.content = self.content.clone();
    cloned.store_name = self.store_name;
    cloned.edited = self.edited;
    cloned
  }

  pub fn append_left(&mut self, str: &str) -> &Self {
    self.outro = concat_string!(self.outro, str);
    self
  }

  pub fn append_right(&mut self, str: &str) -> &Self {
    self.intro = concat_string!(self.intro, str);
    self
  }

  pub fn prepend_left(&mut self, str: &str) -> &Self {
    self.outro = concat_string!(str, self.outro);
    self
  }

  pub fn prepend_right(&mut self, str: &str) -> &Self {
    self.intro = concat_string!(str, self.intro);
    self
  }

  pub fn contains(&self, index: u32) -> bool {
    index >= self.start && index < self.end
  }

  pub fn reset(&mut self) {
    self.intro = String::default();
    self.outro = String::default();
    if self.edited {
      self.edited = false;
      self.content = self.original.clone();
      self.store_name = false;
    }
  }

  pub fn edit(&mut self, content: &str, store_name: bool, content_only: bool) -> &Self {
    self.content = content.to_string();
    if !content_only {
      self.intro.clear();
      self.outro.clear();
    }
    self.store_name = store_name;
    self.edited = true;
    self
  }

  pub fn trim_start(&mut self, reg: &Regex) -> bool {
    self.intro = reg.replace(&self.intro, "").to_string();
    if !self.intro.is_empty() {
      return true;
    }
    let trimmed = reg.replace(&self.content, "").to_string();
    if !trimmed.is_empty() {
      if trimmed != self.content {
        // TODO: aligned with magic-string
        // let new_chunk = Chunk::split(
        //   Rc::new(RefCell::new(self.clone())),
        //   self.end - trimmed.len() as u32,
        // )
        // .expect("trimmed new chunk");
        // if self.edited {
        //   new_chunk.borrow_mut().edit(&trimmed, self.store_name, true);
        // }
        self.edit(trimmed.as_str(), self.store_name, true);
      }
      return true;
    } else {
      self.edit("", self.store_name, true);
      self.outro = reg.replace(&self.outro, "").to_string();
      if !self.outro.is_empty() {
        return true;
      }
    }
    false
  }

  pub fn trim_end(&mut self, reg: &Regex) -> bool {
    self.outro = reg.replace(&self.outro, "").to_string();
    if !self.outro.is_empty() {
      return true;
    }
    let trimmed = reg.replace(&self.content, "").to_string();
    if !trimmed.is_empty() {
      if trimmed != self.content {
        // TODO: aligned with magic-string
        // let new_chunk = Chunk::split(
        //   Rc::new(RefCell::new(self.clone())),
        //   self.end - trimmed.len() as u32,
        // )
        // .expect("trimmed new chunk");
        // if self.edited {
        //   new_chunk.borrow_mut().edit(&trimmed, self.store_name, true);
        // }
        self.edit(trimmed.as_str(), self.store_name, true);
      }
      return true;
    } else {
      self.edit("", self.store_name, true);
      self.intro = reg.replace(&self.intro, "").to_string();
      if !self.intro.is_empty() {
        return true;
      }
    }
    false
  }

  pub fn split(chunk: Rc<RefCell<Chunk>>, index: u32) -> Result<Rc<RefCell<Chunk>>> {
    let mut cur_chunk = chunk.borrow_mut();

    if index < cur_chunk.start {
      return Err(Error::from_reason(
        FmsErrType::Range,
        "index larger than chunk start",
      ));
    }

    // split str
    let mid_index = (index - cur_chunk.start) as usize;
    let origin_before = cur_chunk.original[0..mid_index].to_owned();
    let origin_after = cur_chunk.original[mid_index..].to_owned();

    // create new chunk
    let new_chunk: Rc<RefCell<Chunk>> = Rc::new(RefCell::new(Chunk::new(
      index,
      cur_chunk.end,
      &origin_after,
    )));
    new_chunk.borrow_mut().outro = cur_chunk.outro.to_owned();
    new_chunk.borrow_mut().next = {
      if cur_chunk.next.is_some() {
        Some(Rc::clone(cur_chunk.next.as_ref().unwrap()))
      } else {
        None
      }
    };

    new_chunk.borrow_mut().previous = Some(Rc::clone(&chunk));

    // update current chunk
    cur_chunk.original = origin_before.to_string();
    cur_chunk.content = cur_chunk.original.clone();
    cur_chunk.end = index;
    cur_chunk.outro.clear();
    if cur_chunk.next.is_some() {
      cur_chunk.next.as_mut().unwrap().borrow_mut().previous = Some(new_chunk.clone());
    }

    // weird logic from 'magic-string'
    // For me, the logic makes `overwrite after remove` content correct
    // ```js
    //    const snippet = s.snip(0, 6)
    //    snippet.overwrite(6, 9, 'GHI')
    // ```
    if cur_chunk.is_edited() {
      new_chunk.borrow_mut().edit("", false, false);
      cur_chunk.content = "".to_string();
    }
    cur_chunk.next = Some(Rc::clone(&new_chunk));

    Ok(new_chunk)
  }

  pub fn each_next<F>(chunk: Rc<RefCell<Chunk>>, mut f: F) -> Result<()>
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> Result<bool>,
  {
    let mut cur = Some(chunk);
    while let Some(c) = cur {
      match f(Rc::clone(&c)) {
        Ok(finish) => {
          if finish {
            break;
          }
        }
        Err(e) => return Err(e),
      }
      cur = c.borrow().next.as_ref().map(Rc::clone);
    }
    Ok(())
  }

  pub fn each_previous<F>(chunk: Rc<RefCell<Chunk>>, mut f: F) -> Result<()>
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> Result<bool>,
  {
    let mut cur = Some(chunk);
    while let Some(c) = cur {
      match f(Rc::clone(&c)) {
        Ok(finish) => {
          if finish {
            break;
          }
        }
        Err(e) => return Err(e),
      }
      cur = c.borrow().previous.as_ref().map(Rc::clone);
    }
    Ok(())
  }
}
