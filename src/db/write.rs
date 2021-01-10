use std::sync::Mutex;
use serde_json::Value;
use crate::api::{token::Token, verbs::Executable};

use super::Collection;

pub struct Write;

impl Executable for Write {
  fn exec(&self, src: &mut Value, key: &Vec<Token>, tar: &mut Collection) {
    
  }
}

pub struct Delete;

impl Executable for Delete {
  fn exec(&self, src: &mut Value, key: &Vec<Token>, tar: &mut Collection) {

  }
}