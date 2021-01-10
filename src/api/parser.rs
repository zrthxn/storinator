use crate::api::token::{Token, WhitespaceTokenizer, CharTokenizer};
use crate::api::{Query, Action, Modifier, verbs::{Verb, Specifier}};

pub struct QueryParser<'q> {
  query: &'q str,
  tokens: CharTokenizer<'q>
}

impl<'q> QueryParser<'q> {
  pub fn new(query: &'q str) -> Self {
    QueryParser {
      query: query,
      tokens: WhitespaceTokenizer.tokenize(query)
    }
  }

  pub fn build(self) -> Query<'q> {
    let mut actions: Vec<Action> = Vec::new();
    let mut _target: &mut Vec<Token> = &mut Vec::new();

    for item in self.tokens {
      if is_verb(&item) {
        actions.push(
          Action::new(Verb::from_token(&item), Vec::new(), Vec::new())
        );
        _target = &mut (actions.last_mut().unwrap()).filter;
      } else if is_mod(&item) {
        (actions.last_mut().unwrap()).modifiers.push(
          Modifier::new(Specifier::from_token(&item), Vec::new())
        );
        _target = &mut ((actions.last_mut().unwrap()).modifiers.last_mut().unwrap()).filter;
      } else {
        _target.push(item);
      }
    }

    Query {
      request: self.query,
      actions: actions
    }
  }
}

#[inline]
fn is_verb(item: &Token) -> bool {
  match item.term() {
    "READ"    => true,
    "WRITE"   => true,
    "DELETE"  => true,
    _ => false
  }
}

#[inline]
fn is_mod(item: &Token) -> bool {
  match item.term() {
    "TO"    => true,
    "FROM"  => true,
    "AT"    => true,
    "IN"    => true,
    "WHERE" => true,
    "LIMIT" => true,
    _ => false
  }
}

impl Verb {
  fn from_token(item: &Token) -> Self {
    match item.term() {
      "READ"    => Verb::READ,
      "WRITE"   => Verb::WRITE,
      "DELETE"  => Verb::DELETE,
      _ => Verb::NOP
    }
  }
}

impl Specifier {
  fn from_token(item: &Token) -> Self {
    match item.term() {
      "TO"    => Specifier::TO,
      "FROM"  => Specifier::FROM,
      "AT"    => Specifier::AT,
      "IN"    => Specifier::IN,
      "WHERE" => Specifier::WHERE,
      "LIMIT" => Specifier::LIMIT,
      _ => Specifier::NOP
    }
  }
}