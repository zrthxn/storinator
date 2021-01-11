use serde_json::{Value, json};
use crate::api::{token::Token, verbs::Executable};

use super::Collection;

pub struct Write;

impl Executable for Write {
  fn exec(&self, src: &mut Value, keys: &Vec<Token>, tar: &mut Collection) {
    let mut doc = String::new();
    for piece in keys {
      doc.push_str(piece.term());
      if piece.term() != "}" {
        doc.push_str(" ");
      }
    }

    let mut document = json!({ "insert": Value::from(doc) });

    let mut payload = src["data"]["%_target_%"].as_object_mut().unwrap().clone();
    let target = src["data"].as_object_mut().unwrap().clone();

    for (_k, marker) in target {
      if marker == Value::from("%_target_%") {
        for (key, value) in document.as_object_mut().unwrap().clone() {
          payload.insert(key.to_string(), value.clone());
        }

        src["data"][_k] = Value::Object(payload.clone());
        src["data"]["%_target_%"] = Value::Null;
        break;
      }
    }

    tar.val = src.clone();
  }
}

impl Write {
  pub fn to_collection(&self, src: &mut Value, keys: &Vec<Token>, _t: &mut Collection) {
    let index = keys[0].term();
    src["data"]["%_target_%"] = src["data"][index].clone();
    src["data"][index] = Value::from("%_target_%");
  }
}

pub struct Delete;

impl Executable for Delete {
  fn exec(&self, src: &mut Value, keys: &Vec<Token>, tar: &mut Collection) {

  }
}