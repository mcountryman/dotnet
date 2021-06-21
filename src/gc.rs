use std::{marker::PhantomData, ptr::NonNull};

use crate::Host;

#[derive(Debug)]
pub struct GcHandle<T, H: Host> {
  ptr: NonNull<T>,
  phantom: PhantomData<H>,
}

impl<T, H: Host> Drop for GcHandle<T, H> {
  fn drop(&mut self) {
    if let Ok(host) = H::get() {
      host.release(self);
    }
  }
}
