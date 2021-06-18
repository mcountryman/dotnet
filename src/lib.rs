pub mod class;
pub mod exception;
pub mod gc;
pub mod marshal;
pub mod method;
pub mod types;

use gc::GcHandle;
use method::Method;
use std::error::Error;

pub trait Host: Sized {
  type Error: Error;

  fn get() -> Result<Self, Self::Error>;

  fn method<M, A>(&self, path: &str) -> Result<M, Self::Error>
  where
    M: Method<A>,
    // Not required for this to work but, prevents returning non-plain `fn(..) -> ..` fns by
    // requiring return to be `Fn(..) -> ..`
    M::Fn: Method<A>;

  fn release<T>(&self, handle: GcHandle<T, Self>) -> Result<M, Self::Error>;
}
