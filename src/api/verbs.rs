use crate::db::Collection;
use crate::api::token::Token;
use crate::api::Modifier;

/// Properties that Verbs must have
pub trait Executable {
  fn exec(t: Vec<Token>) -> Collection;
}

pub struct Read;

impl Executable for Read {
  fn exec(target: Vec<Token>) -> Collection {
    Collection {}
  }
}

pub struct Write;

impl Executable for Write {
  fn exec(target: Vec<Token>) -> Collection {
    Collection {}
  }
}

pub struct Delete;

impl Executable for Delete {
  fn exec(target: Vec<Token>) -> Collection {
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

impl PartialEq for Verb {
  fn eq(&self, other: &Self) -> bool {
    if self == other {
      true
    } else {
      false
    }
  }

  fn ne(&self, other: &Self) -> bool {
    if self == other {
      false
    } else {
      true
    }
  }
}


pub enum Specifier {
  TO, FROM, IN, AT, WHERE, LIMIT, 
  NOP
}

impl PartialEq for Specifier {
  fn eq(&self, other: &Self) -> bool {
    if self == other {
      true
    } else {
      false
    }
  }

  fn ne(&self, other: &Self) -> bool {
    if self == other {
      false
    } else {
      true
    }
  }
}