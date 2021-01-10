use crate::db::Collection;
use crate::api::token::Token;

/// Properties that Verbs must have
pub trait Executable {
  // fn exec(&self, t: Vec<Token>) -> Collection;
  fn exec(&self, filter: &Vec<Token>, target: &mut Collection) -> Collection;
}

pub struct Read;

impl Executable for Read {
  fn exec(&self, keys: &Vec<Token>, tar: &mut Collection) -> Collection {
    Collection::empty()
  }
}

pub struct Write;

impl Executable for Write {
  fn exec(&self, keys: &Vec<Token>, tar: &mut Collection) -> Collection {
    Collection::empty()
  }
}

pub struct Delete;

impl Executable for Delete {
  fn exec(&self, keys: &Vec<Token>, tar: &mut Collection) -> Collection {
    Collection::empty()
  }
}

/// Encapsulating all Verbs
pub enum Verb {
  READ, WRITE, DELETE, // JOIN
  NOP
}

impl Executable for Verb {
  fn exec(&self, f: &Vec<Token>, t: &mut Collection) -> Collection {
    match self {
      Self::READ    => Read.exec(f, t),
      Self::WRITE   => Write.exec(f, t),
      Self::DELETE  => Delete.exec(f, t),
      _ => Collection::empty()
    }
  }
}

/// Encapsulating all Modifiers
pub enum Specifier {
  TO, FROM, IN, AT, WHERE, LIMIT, 
  NOP
}

impl Executable for Specifier {
  fn exec(&self, f: &Vec<Token>, t: &mut Collection) -> Collection {
    match self {
      Self::FROM => Collection::empty(),
      _ => Collection::empty()
    }
  }
}
