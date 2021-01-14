use serde_json::Value;

use crate::api::token::Token;
use crate::db::Collection;
use crate::db::read::Read;
use crate::db::write::{Write, Delete};

/// Properties that Verbs must have
pub trait Executable {
  fn exec(&self, source: &mut Value, filter: &Vec<Token>, target: &mut Collection);
}

/// Encapsulating all Verbs
pub enum Verb {
  READ, WRITE, DELETE, // JOIN
  NOP
}

impl Executable for Verb {
  fn exec(&self, s: &mut Value, f: &Vec<Token>, t: &mut Collection) {
    match self {
      Self::READ    => Read.exec(s, f, t),
      Self::WRITE   => Write.exec(s, f, t),
      Self::DELETE  => Delete.exec(s, f, t),
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
  fn exec(&self, s: &mut Value, f: &Vec<Token>, t: &mut Collection) {
    match self {
      Self::FROM  => Read.from_collection(s, f, t),
      Self::TO    => Write.to_collection(s, f, t),
      _ => ()
    }
  }
}
