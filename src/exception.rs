use crate::{class::Class, Host};
use std::ffi::c_void;

#[derive(Clone)]
pub struct Exception<H: Host>(Class<H>);

impl<H: Host> Exception<H> {
  pub unsafe fn new_unchecked(ptr: *mut c_void) -> Result<Self, H::Error> {
    Ok(Self(Class::new(ptr)?))
  }
}
