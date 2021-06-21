use crate::{runtime::Global, Runtime};
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
pub struct GcHandle<T, R: Runtime = Global> {
  ptr: NonNull<T>,
  phantom: PhantomData<R>,
}

impl<T, R: Runtime> Drop for GcHandle<T, R> {
  fn drop(&mut self) {
    if let Ok(rt) = R::get() {
      rt.release(&mut self.ptr)
        .expect("Failed to release GcHandle")
    }
  }
}
