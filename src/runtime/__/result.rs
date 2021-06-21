use crate::Exception;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HostResult<T: Clone> {
  pub value: T,
}
