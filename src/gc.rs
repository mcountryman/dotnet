use crate::{runtime::Global, Runtime};
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
pub struct GcHandle<T, R: Runtime = Global> {
  ptr: NonNull<T>,
  phantom: PhantomData<R>,
}

impl<T, H: Runtime> Drop for GcHandle<T, H> {
  fn drop(&mut self) {
    if let Ok(host) = H::get() {
      host.release(self);
    }
  }
}
