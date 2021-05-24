use crate::Exception;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HostResult<T: Clone> {
  value: T,
}
