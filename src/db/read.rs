use serde_json::Value;

use crate::api::{token::Token, verbs::Executable};

use super::Collection;

pub struct Read;

impl Executable for Read {
  fn exec(&self, _s: &mut Value, keys: &Vec<Token>, tar: &mut Collection) {
    let index = keys[0].term();
    if index != "ALL" {
      tar.val = tar.val[index].clone();
    }
  }
}

impl Read {
  pub fn from_collection(&self, src: &mut Value, keys: &Vec<Token>, tar: &mut Collection) {
    let index = keys[0].term();
    tar.val = src["data"].clone();
    if index != "ALL" {
      tar.val = tar.val[index].clone();
    }
  } 
}