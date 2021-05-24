use crate::{HostResult, Object};
use std::{ffi::CString, os::raw::c_char};

type Invoke = unsafe fn(
  //
  path: *const c_char,
  argv: *const Object,
  argc: u16,
) -> HostResult<Object>;

#[repr(C)]
#[derive(Debug)]
pub struct Bridge {
  invoke_static: Invoke,
}

impl Bridge {
  pub fn invoke_static(&self, path: &str, args: &[Object]) -> HostResult<Object> {
    let path = CString::new(path).unwrap();
    let path = path.as_ptr();

    let argc = args.len() as _;
    let argv = args.as_ptr();

    unsafe { (self.invoke_static)(path, argv, argc) }
  }
}
