use crate::{
  marshal::{Marshal, MarshalError, MarshalFrom, MarshalTo},
  types::{Type, TypeId},
  Host,
};
use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct Class<H: Host> {
  ptr: NonNull<c_void>,
  phantom: PhantomData<H>,
}

impl<H: Host> Class<H> {
  pub fn new(ptr: *mut c_void) -> Result<Self, H::Error> {
    Ok(Self {
      ptr: NonNull::new(ptr).unwrap(),
      phantom: Default::default(),
    })
  }

  pub fn get_field<M: MarshalFrom>(&self, name: &str) -> Result<M, H::Error> {
    todo!()
  }

  pub fn get_property<M: MarshalFrom>(&self, name: &str) -> Result<M, H::Error> {
    todo!()
  }

  pub fn get_type(&mut self) -> Result<Type<H>, H::Error> {
    todo!()
  }
}

impl<H: Host> Marshal for Class<H> {
  type Managed = *mut c_void;

  fn id() -> TypeId {
    TypeId::Object
  }
}

impl<H: Host> MarshalTo for Class<H> {
  fn marshal_to(self) -> Result<Self::Managed, MarshalError> {
    todo!()
  }
}

impl<H: Host> MarshalFrom for Class<H> {
  fn marshal_from(from: Self::Managed) -> Result<Self, MarshalError> {
    todo!()
  }
}
