use crate::api::token::WhitespaceTokenizer;

fn test_parser() {
  WhitespaceTokenizer.tokenize("input: &'wt str")
}