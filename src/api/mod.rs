pub mod token;
pub mod parser;

use tracing::{info, instrument};
use token::WhitespaceTokenizer;
use parser::QueryParser;

pub enum Verb {
  READ, WRITE, DELETE, JOIN
}

pub enum Specifier {
  TO, FROM, IN, AT
}

pub enum Modifier {
  WHERE, LIMIT
}

pub struct Query<'q> {
  reqest: &'q str,
  action: Verb,
}

// pub enum TOKENS {
//   WRITE D TO C
//   READ D FROM C WHERE Q LIMIT L
//   UPDATE D IN C WHERE Q LIMIT L
//   DELETE D FROM C WHERE Q LIMIT L

//   VERB (REF) LOCATOR (REF) QUERY

//   (C) JOIN AT Q (C) 
// }

// // WRITE username = 'myraduser' TO users WHERE uuid === '1203724981'

// // READ uuid FROM users WHERE accountAge > 3000 AND userAge < 50
// // WRITE username = 'deleted' TO users WHERE uuid # ['...', '...', '...', '...']

#[instrument]
pub fn execute() {
  info!("Executing query");
  let testtoken = "test";
  let tok = token::Token::from_str(&testtoken, 0, 0);
  println!("The token is {}", tok.term());

  let reqst = "READ D FROM C WHERE Q";
  let parse = QueryParser::new(reqst);
  parse.build();
}