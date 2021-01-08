pub mod read;

pub trait Findable {
  fn find() -> String;
}

pub struct Collection {
  id: String
}