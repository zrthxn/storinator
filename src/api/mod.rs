pub mod token;
pub mod parser;
pub mod verbs;

use serde_json::Value;
use parser::QueryParser;
use verbs::{Verb, Specifier, Executable};
use token::{Token};

use crate::db::Collection;

pub struct Action {
  verb: Verb,
  filter: Vec<Token>,
  modifiers: Vec<Modifier>
}

impl Action {
  pub fn new(verb: Verb, filter: Vec<Token>, mods: Vec<Modifier>) -> Self {
    Action { verb: verb, filter: filter, modifiers: mods }
  }
}

pub struct Modifier {
  spec: Specifier,
  filter: Vec<Token>
}

impl Modifier {
  pub fn new(spec: Specifier, filter: Vec<Token>) -> Self {
    Modifier { spec: spec, filter: filter }
  }
}

/// Query structure for DB interaction
pub struct Query {
  request: String,
  actions: Vec<Action>
}

impl Query {
  pub fn run(self, source: &mut Value, collection: &mut Collection) {
    // + sort actions by order of predecence
    
    for action in self.actions {
      for modifier in action.modifiers {
        modifier.spec.exec(source, &modifier.filter, collection);
      }
      action.verb.exec(source, &action.filter, collection);
    }
  }
}

pub fn parse(query: &str) -> Query {
  let parser = QueryParser::new(query);
  parser.build()
}

// pub enum TOKENS {
//   WRITE doc TO collection 
//   READ doc FROM collection WHERE Q LIMIT L
//   READ doc FROM collection.key WHERE Q LIMIT L
//   UPDATE doc IN collection WHERE Q LIMIT L
//   UPDATE doc IN collection.key WHERE Q LIMIT L
//   DELETE doc FROM collection WHERE Q LIMIT L

//   VERB (REF) LOCATOR (REF) QUERY

//   (C) JOIN AT Q (C) 
// }

// // WRITE username = 'myraduser' TO users WHERE uuid === '1203724981'

// // READ uuid FROM users WHERE accountAge > 3000 AND userAge < 50
// // WRITE username = 'deleted' TO users WHERE uuid # ['...', '...', '...', '...']
