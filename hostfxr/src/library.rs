use std::error::Error;
use std::ffi::{CString, OsStr};
use std::ops::Deref;
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};

#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::unix::{Library, Symbol};
use owning_ref::{BoxRef, OwningRef};

use crate::types::{
  CloseFn, DelegateType, FxrContextHandle, GetRuntimeDelegateFn, InitializeForRuntimeConfigFn,
  InitializeParameters, LoadAssemblyAndGetFunctionPointer,
};
use std::mem;

pub struct HostFxrLibrary {
  library: Box<Library>,
  init_symbol: Symbol<InitializeForRuntimeConfigFn>,
  close_symbol: Symbol<CloseFn>,
  get_delegate_symbol: Symbol<GetRuntimeDelegateFn>,
}

impl HostFxrLibrary {
  pub fn new<P: AsRef<OsStr>>(filename: P) -> Result<Self, Box<dyn Error>> {
    let library = Library::new(filename)?;
    let library = Box::new(library);

    let init_symbol;
    let close_symbol;
    let get_delegate_symbol;

    unsafe {
      init_symbol = library.get(b"hostfxr_initialize_for_runtime_config")?;
      close_symbol = library.get(b"hostfxr_close")?;
      get_delegate_symbol = library.get(b"hostfxr_get_runtime_delegate")?;
    }

    Ok(Self {
      library,
      init_symbol,
      close_symbol,
      get_delegate_symbol,
    })
  }

  pub fn initialize<S: Into<String>>(
    &self,
    runtime_config_path: S,
  ) -> Result<FxrContextHandle, Box<dyn Error>> {
    let mut result = 0;
    let mut handle = null_mut();
    let runtime_config_path = CString::new(runtime_config_path.into())?;
    let runtime_config_path = runtime_config_path.as_ptr();
    let initialize = self.init_symbol.deref();
    let close = self.close_symbol.deref();

    unsafe {
      result = initialize(runtime_config_path, null_mut(), &mut handle);

      if result < 0 {
        close(handle);
        bail!("Failed to initialize HostFxrContext {}", result);
      }
    }

    Ok(handle)
  }

  pub unsafe fn close(&self, host_context_handle: FxrContextHandle) -> i32 {
    self.close_symbol.deref()(host_context_handle)
  }

  pub unsafe fn get_load_assembly_fn(
    &self,
    host_context_handle: FxrContextHandle,
  ) -> Result<LoadAssemblyAndGetFunctionPointer, Box<dyn Error>> {
    let mut load_assembly_fn = null_mut();
    let get_runtime_delegate = self.get_delegate_symbol.deref();
    let result = get_runtime_delegate(
      host_context_handle,
      DelegateType::LoadAssemblyAndGetFunctionPointer,
      &mut load_assembly_fn,
    );

    if result < 0 {
      bail!("Failed to get load_assembly_fn {}", result);
    }

    Ok(mem::transmute(load_assembly_fn))
  }

  unsafe fn get_runtime_delegate(
    self,
    host_context_handle: FxrContextHandle,
    type_: DelegateType,
    delegate: *mut *mut c_void,
  ) -> i32 {
    self.get_delegate_symbol.deref()(host_context_handle, type_, delegate)
  }
}
