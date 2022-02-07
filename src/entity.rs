pub trait Collection {
  type Id;

  fn id(&self) -> Self::Id;

  fn store(&self) {
    // To impl
    println!("Writing to disk");
  }
}
