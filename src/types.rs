use crate::{class::Class, Host};
use std::ops::Deref;

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

#[derive(Clone)]
pub struct Type<H: Host>(Class<H>);

impl<H: Host> Type<H> {
  pub unsafe fn new_unchecked(class: Class<H>) -> Self {
    Self(class)
  }

  pub fn get_name(&self) -> Result<String, H::Error> {
    self.get_property("Name")
  }

  pub fn is_class(&self) -> Result<bool, H::Error> {
    self.get_property("IsClass")
  }
}

impl<H: Host> Deref for Type<H> {
  type Target = Class<H>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
