pub mod token;
pub mod parser;
pub mod verbs;

use tracing::{info, instrument};
use parser::QueryParser;
use verbs::{Verb, Specifier};
use token::{Token};

pub struct Action {
  verb: Verb,
  target: Vec<Token>,
  modifiers: Vec<Modifier>
}

impl Action {
  pub fn new(verb: Verb, target: Vec<Token>, mods: Vec<Modifier>) -> Self {
    Action { verb: verb, target: target, modifiers: mods }
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

pub struct Query<'q> {
  request: &'q str,
  actions: Vec<Action>
}

impl<'q> Query<'q> {
  pub fn execute(self) {
    info!("Executing {} actions", self.actions.len());
    info!("{}", self.request);

    for action in self.actions {
      action.verb.exec(action.target);
      // println!("{}")
    }
  }
}

#[instrument]
pub fn execute(query: &str) {
  info!("Building query");

  let parser = QueryParser::new(query);
  let sequence = parser.build();

  sequence.execute();
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
