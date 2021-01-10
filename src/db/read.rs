use std::fs;
use std::sync::Mutex;
use std::time::{Instant};
use serde_json::Value;

use crate::api::{token::Token, verbs::Executable};

use super::Collection;

pub struct Read;

impl Executable for Read {
  fn exec(&self, src: &mut Value, keys: &Vec<Token>, tar: &mut Collection) {
    
  }
}

impl Read {
  pub fn from_collection(&self, src: &mut Value, key: &Vec<Token>, tar: &mut Collection) {
    if key.len() > 1 {
      // Err("More than one values for `collection`");
    }
  } 
}