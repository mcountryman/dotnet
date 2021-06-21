use std::ops::Deref;

use crate::{class::Class, gc::GcHandle, Host};

#[derive(Debug)]
pub struct Exception<H: Host>(Class<H>);

impl<H: Host> Exception<H> {
  pub unsafe fn new_unchecked(handle: GcHandle<(), H>) -> Self {
    Self(Class::new(handle))
  }

  pub fn message(&self) -> Result<String, H::Error> {
    self.get_property("Message")
  }
}

impl<H: Host> Deref for Exception<H> {
  type Target = Class<H>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
