use std::fs;
use std::time::{Instant};
use serde_json::{Value, json};

/// Test
pub fn read_db() {
  let start1 = Instant::now();

  let data = json!({
    "uuid": {  }
  });
  
  let raw = fs::read_to_string("store.json")
    .expect("Error in reading file");

  let start2 = Instant::now();
  let mut keys: Value = serde_json::from_str(&raw).unwrap();
  keys["store"]["data"] = data;
  println!("{:?} elapsed in mem op", start2.elapsed());

  fs::write("store.json", keys.to_string()).unwrap();

  println!("{:?} elapsed in file op", start1.elapsed());
}

pub fn read_key() {
  
}