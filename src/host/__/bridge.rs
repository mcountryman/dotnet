use crate::{HostResult, ObjectKind};
use std::{
  ffi::{CStr, CString},
  os::raw::c_char,
};

type PrepareInvoke = unsafe extern "stdcall" fn(
  path: *const c_char,
  ret: ObjectKind,
  argv: *const ObjectKind,
  argc: u16,
) -> HostResult<*const u8>;

#[repr(C)]
#[derive(Clone)]
pub struct Bridge {
  prepare_invoke: PrepareInvoke,
}

impl Bridge {
  pub fn prepare_invoke<F>(
    &self,
    path: &str,
    ret: ObjectKind,
    args: &[ObjectKind],
  ) -> &F {
    let path = CString::new(path).unwrap();
    let path = path.into_raw();

    let argc = args.len() as _;
    let argv = args.as_ptr();

    let ptr = unsafe {
      let result = (self.prepare_invoke)(path, ret, argv, argc);
      &*(result.value as *const F)
    };

    ptr
  }
}
