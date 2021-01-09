use crate::db::Collection;
use crate::api::token::Token;

/// Properties that Verbs must have
pub trait Executable {
  fn exec(t: Vec<Token>) -> Collection;
}

pub struct Read;

impl Executable for Read {
  fn exec(target: Vec<Token>) -> Collection {
    // println!("DB READ");
    Collection {}
  }
}

pub struct Write;

impl Executable for Write {
  fn exec(target: Vec<Token>) -> Collection {
    println!("DB WRITE");
    Collection {}
  }
}

pub struct Delete;

impl Executable for Delete {
  fn exec(target: Vec<Token>) -> Collection {
    println!("DB DELETE");
    Collection {}
  }
}

/// Encapsulating all Verbs
pub enum Verb {
  READ, WRITE, DELETE, // JOIN
  NOP
}

impl Verb {
  pub fn exec(&self, t: Vec<Token>) -> Collection {
    match self {
      Self::READ => Read::exec(t),
      Self::WRITE => Write::exec(t),
      Self::DELETE => Delete::exec(t),
      _ => Collection {}
    }
  }
}

/// Encapsulating all Modifiers
pub enum Specifier {
  TO, FROM, IN, AT, WHERE, LIMIT, 
  NOP
}
