pub mod read;
pub mod write;

use std::{str::FromStr, sync::Mutex};
use serde_json::{Value, json};
use serde::Deserialize;

use crate::api::Query;

#[derive(Deserialize)]
pub struct DataStore {
  pub store: Mutex<Value>
}

pub struct Collection {
  pub key: String,
  pub val: Value
}

impl Collection {
  pub fn new(key: &str, value: Value) -> Self {
    Collection {
      key: String::from_str(key).unwrap(),
      val: value
    }
  }
}

pub fn execute(query: Query, store: &Mutex<Value>) -> Collection {
  let mut data = &mut *store.lock().unwrap();
  let mut target = Collection::new("result", json!({}));

  query.run(&mut data, &mut target);
  return target;
}