use tracing::{info, instrument};
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
    let mut _target: Vec<Token> = Vec::new();

    let mut _action = Action::new(Verb::NOP, Vec::new(), Vec::new());
    let mut _modif = Modifier::new(Specifier::NOP, Vec::new());

    for item in self.tokens {      
      if is_verb(&item) {
        if _action.verb != Verb::NOP {
          _action.target = _target;
          actions.push(_action);
          
          _action = Action::new(Verb::NOP, Vec::new(), Vec::new());
          _target = Vec::new();
        }

        _action.verb = verb_from_token(&item);
        continue;
      } 
      
      if is_mod(&item) {
        if _modif.spec != Specifier::NOP {
          _modif.filter = _target;
          _action.modifiers.push(_modif);
          
          _modif = Modifier::new(Specifier::NOP, Vec::new());
          _target = Vec::new();
        }

        _modif.spec = mod_from_token(&item);
        continue;
      } 
      
      _target.push(item);
    }

    // if _modif.spec != Specifier::NOP {
    //   _modif.filter.copy_from_slice(&_target);
    //   _action.modifiers.push(_modif);
    // }

    // if _action.verb != Verb::NOP {
    //   _action.target.copy_from_slice(&_target);
    //   actions.push(_action);
    // }

    Query {
      request: self.query,
      actions: actions
    }
  }
}

#[inline]
fn is_verb(item: &Token) -> bool {
  match item.term() {
    "READ" => true,
    "WRITE" => true,
    "DELETE" => true,
    "END" => true,
    _ => false
  }
}

#[inline]
fn verb_from_token(item: &Token) -> Verb {
  match item.term() {
    "READ" => Verb::READ,
    "WRITE" => Verb::WRITE,
    "DELETE" => Verb::DELETE,
    _ => Verb::NOP
  }
}

#[inline]
fn is_mod(item: &Token) -> bool {
  match item.term() {
    "FROM" => true,
    _ => false
  }
}

#[inline]
fn mod_from_token(item: &Token) -> Specifier {
  match item.term() {
    "FROM" => Specifier::FROM,
    _ => Specifier::NOP
  }
}