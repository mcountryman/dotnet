use crate::{
  gc::GcHandle,
  marshal::{Marshal, MarshalError, MarshalFrom, MarshalTo},
  runtime::Global,
  types::{Type, TypeId},
  Runtime,
};
use std::{ffi::c_void, marker::PhantomData};

#[derive(Debug)]
pub struct Class<R: Runtime = Global> {
  handle: GcHandle<(), R>,
  phantom: PhantomData<R>,
}

impl<R: Runtime> Class<R> {
  pub fn new(handle: GcHandle<(), R>) -> Self {
    Self {
      handle,
      phantom: Default::default(),
    }
  }

  pub fn get_field<M: MarshalFrom>(&self, name: &str) -> Result<M, R::Error> {
    todo!()
  }

  pub fn get_property<M: MarshalFrom>(&self, name: &str) -> Result<M, R::Error> {
    todo!()
  }

  pub fn get_type(&mut self) -> Result<Type<R>, R::Error> {
    todo!()
  }
}

impl<R: Runtime> Marshal for Class<R> {
  type Managed = *mut c_void;

  fn id() -> TypeId {
    TypeId::Object
  }
}

impl<R: Runtime> MarshalTo for Class<R> {
  fn marshal_to(self) -> Result<Self::Managed, MarshalError> {
    todo!()
  }
}

impl<R: Runtime> MarshalFrom for Class<R> {
  fn marshal_from(from: Self::Managed) -> Result<Self, MarshalError> {
    todo!()
  }
}
