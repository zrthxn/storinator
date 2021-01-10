pub mod read;

use std::{fs, str::FromStr, sync::Mutex};
use serde_json::Value;
use serde::Deserialize;
use crate::api::Query; 

#[derive(Deserialize)]
pub struct DataStore {
  pub store: Mutex<serde_json::Value>
}

pub struct Collection {
  key: String,
  // data: Vec<String>
}

impl Collection {
  pub fn new(key: &str) -> Self {
    Collection {
      key: String::from_str(key).unwrap() 
    }
  }

  pub fn empty() -> Self {
    Collection {
      key: String::from_str("").unwrap()
    }
  }
}

pub fn execute(query: Query, store: &Mutex<Value>) -> Collection {

  Collection::empty()
}