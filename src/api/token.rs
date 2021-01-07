// use std::ops::A
use arrayvec::ArrayString;

const MAX_STACK_TERM_LEN: usize = 15;
enum Term {
  Stack(ArrayString<[u8; MAX_STACK_TERM_LEN]>),
  Heap(String)
}

/// Properties of a Token
pub struct Token {
  term: Term,
  position: usize,
  offset: usize,
}

impl Token {
  #[inline]
  pub fn from_str(term: &str, position: usize, offset: usize) -> Self {
    Token {
      term: Token::convert_term(term),
      position: position,
      offset: offset
    }
  }

  #[inline]
  fn convert_term(term: &str) -> Term {
    if term.len() <= MAX_STACK_TERM_LEN {
      Term::Stack(ArrayString::<[_; MAX_STACK_TERM_LEN]>::from(term).unwrap())
    } else {
      Term::Heap(term.to_string())
    }
  }

  #[inline]
  pub fn term(&self) -> &str {
    match self.term {
      Term::Heap(ref s) => s.as_ref(),
      Term::Stack(ref s) => s.as_ref(),
    }
  }
}

/// Required properties of Tokenizers
// pub trait Tokenizer<'t> {
//   type TokenIter = dyn Iterator<Item = Token>;
//   fn tokenize(&self, input: &'t str) -> Self::TokenIter;
// }

pub struct CharTokenizer<'ct> {
  filter: fn(&(usize, (usize, char))) -> bool,
  input: &'ct str,
  byte_offset: usize,
  char_offset: usize,
  position: usize,
}

impl<'ct> CharTokenizer<'ct> {
  pub fn new(filter: fn(&(usize, (usize, char))) -> bool, input: &'ct str) -> Self {
    CharTokenizer { filter: filter, input: input, byte_offset: 0, char_offset: 0, position: 0 }
  }
}

impl<'a> Iterator for CharTokenizer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let mut skipped_bytes = 0;
    let mut skipped_chars = 0;

    for (cidx, (bidx, c)) in self.input[self.byte_offset..]
      .char_indices()
      .enumerate()
      .filter(&self.filter) {
        let char_len = c.len_utf8();

        if cidx - skipped_chars == 0 {
          self.byte_offset = self.byte_offset + char_len;
          self.char_offset += 1;
          skipped_bytes = skipped_bytes + char_len;
          skipped_chars += 1;
          continue;
        }

        let slice = &self.input[self.byte_offset..self.byte_offset + bidx - skipped_bytes];
        self.char_offset = self.char_offset + slice.chars().count() + 1;
        self.position += 1;
        self.byte_offset = self.byte_offset + bidx + char_len - skipped_bytes;
        
        return Some(Token::from_str(slice, self.char_offset, self.position));
      }

    if self.byte_offset < self.input.len() {
      let slice = &self.input[self.byte_offset..];
      self.byte_offset = self.input.len();
      Some(Token::from_str(slice, self.char_offset, self.position))
    } else {
      None
    }
  }
}

/// Whitespace tokenizer using the CharTokenIter
pub struct WhitespaceTokenizer;

impl<'wt> WhitespaceTokenizer {
  pub fn tokenize(&self, input: &'wt str) -> CharTokenizer<'wt> {
    CharTokenizer::new(is_whitespace, input)
  }
}

#[inline]
fn is_whitespace(input: &(usize, (usize, char))) -> bool {
  let (_, (_, c)) = *input;
  c.is_whitespace()
}