use crate::{class::Class, gc::GcHandle, runtime::Global, Runtime};
use std::ops::Deref;

#[derive(Debug)]
pub struct Exception<R: Runtime = Global>(Class<R>);

impl<R: Runtime> Exception<R> {
  pub unsafe fn new_unchecked(handle: GcHandle<(), R>) -> Self {
    Self(Class::new(handle))
  }

  pub fn message(&self) -> Result<String, R::Error> {
    self.get_property("Message")
  }
}

impl<R: Runtime> Deref for Exception<R> {
  type Target = Class<R>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
