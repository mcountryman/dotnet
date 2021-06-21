use crate::{exception::Exception, runtime::Global, Runtime};

#[repr(C, u8)]
pub enum RuntimeResult<T, R: Runtime = Global> {
  Ok(T),
  Err(RuntimeError<R>),
}

pub enum RuntimeError<R: Runtime = Global> {
  Exception(Exception<R>),
}
