use std::{marker::PhantomData, ptr::NonNull};

use crate::Host;

#[derive(Debug, Clone, Copy)]
pub struct GcHandle<T, H: Host> {
  ptr: NonNull<T>,
  phantom: PhantomData<H>,
}

impl<T, H: Host> Drop for GcHandle<T, H> {
  fn drop(&mut self) {
    match H::get() {
      Ok(host) => host.release(self),
      Err(_) => {}
    }
  }
}
