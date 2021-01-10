pub mod read;
pub mod write;

use std::{str::FromStr, sync::Mutex};
use actix_web::web;
use serde_json::{Value, json};
use serde::Deserialize;

use crate::api::Query;

#[derive(Deserialize)]
pub struct DataStore {
  pub store: Mutex<Value>
}

pub struct Collection<'c> {
  key: String,
  val: &'c mut Value
}

impl<'c> Collection<'c> {
  pub fn new(key: &'c str, value: &'c mut Value) -> Self {
    Collection {
      key: String::from_str(key).unwrap(),
      val: value
    }
  }

  // pub fn empty() -> Self {
  //   Collection {
  //     key: String::from_str("").unwrap(),
  //     val: &mut Value::from_str("{}").unwrap()
  //   }
  // }
}

pub fn execute<'e>(query: &Query, store: &Mutex<Value>) {
  let lock = &mut *store.lock().unwrap();
  let mut base = Collection::new("result", lock);

  (*query).run(&mut base);
  // return base.val;
}