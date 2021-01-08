use crate::api::token::{WhitespaceTokenizer, CharTokenizer};
use crate::api::{Verb, Specifier, Modifier, Query};

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

  pub fn build(self) {
    for item in self.tokens {
      println!("{}", item.term());
      // if item.term() = Verb::READ {

      // }
    }
  }
}

fn test_parser() {
  let query = WhitespaceTokenizer.tokenize("READ D FROM C WHERE Q");
}