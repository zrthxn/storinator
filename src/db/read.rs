use std::fs;
use std::time::{Duration, Instant};
use serde_json::{Result, Value, json};

/// Test
pub fn read_db() {
  let start = Instant::now();
  let data = json!({
    "uuid": {  }
  });
  
  let raw = fs::read_to_string("store.json")
    .expect("Error in reading file");

  let mut keys: Value = serde_json::from_str(&raw).unwrap();
  keys["store"]["data"] = data;

  fs::write("store.json", keys.to_string()).unwrap();

  println!("{:?} elapsed in file op", start.elapsed());
}