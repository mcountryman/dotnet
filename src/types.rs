pub trait Type {
  fn name() -> &'static str;
}

impl Type for bool {
  fn name() -> &'static str {
    "System.Boolean"
  }
}
