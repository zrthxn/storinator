use std::sync::Mutex;
use serde_json::Value;

use crate::api::token::Token;
use crate::db::Collection;
use crate::db::read::Read;
use crate::db::write::{Write, Delete};

/// Properties that Verbs must have
pub trait Executable {
  // fn exec(&self, t: Vec<Token>) -> Collection;
  fn exec(&self, filter: &Vec<Token>, target: &mut Collection);
  // fn exec(&self, source: &Mutex<Value>, filter: &Vec<Token>, target: &mut Collection) -> Collection;
}

/// Encapsulating all Verbs
pub enum Verb {
  READ, WRITE, DELETE, // JOIN
  NOP
}

impl Executable for Verb {
  fn exec(&self, f: &Vec<Token>, t: &mut Collection) {
    match self {
      Self::READ    => Read.exec(f, t),
      Self::WRITE   => Write.exec(f, t),
      Self::DELETE  => Delete.exec(f, t),
      _ => ()
    }
  }
}

/// Encapsulating all Modifiers
pub enum Specifier {
  TO, FROM, IN, AT, WHERE, LIMIT, 
  NOP
}

impl Executable for Specifier {
  fn exec(&self, f: &Vec<Token>, t: &mut Collection) {
    match self {
      Self::FROM => Read.from_collection(f, t),
      _ => ()
    }
  }
}
