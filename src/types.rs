use crate::{class::Class, runtime::Global, Runtime};
use std::ops::Deref;

#[repr(C, u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeId {
  Char,
  Byte,
  Int16,
  Int32,
  Int64,

  SByte,
  UInt16,
  UInt32,
  UInt64,

  Float,
  Double,
  String,
  Boolean,

  Object,

  Array(Box<TypeId>),
  Nullable(Box<TypeId>),
  Enumerable(Box<TypeId>),
}

#[derive(Debug)]
pub struct Type<R: Runtime = Global>(Class<R>);

impl<R: Runtime> Type<R> {
  pub unsafe fn new_unchecked(class: Class<R>) -> Self {
    Self(class)
  }

  pub fn get_name(&self) -> Result<String, R::Error> {
    self.get_property("Name")
  }

  pub fn is_class(&self) -> Result<bool, R::Error> {
    self.get_property("IsClass")
  }
}

impl<R: Runtime> Deref for Type<R> {
  type Target = Class<R>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
