use hostfxr_sys::HRESULT;
use std::error::Error;
use std::ffi::{CString, OsStr};
use std::mem;
use std::ops::Deref;
use std::os::raw::{c_char, c_ulong, c_void};
use std::ptr::null_mut;

use crate::hresult::HResult;
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::unix::{Library, Symbol};
use std::path::Path;

pub struct HostFxrLibrary {
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
      init_symbol,
      close_symbol,
      get_delegate_symbol,
    })
  }

  pub fn initialize<P: AsRef<Path>>(
    &self,
    runtime_config_path: P,
  ) -> Result<FxrContextHandle, Box<dyn Error>> {
    let mut handle = null_mut();
    let runtime_config_path = runtime_config_path.as_ref().to_str().unwrap();
    let runtime_config_path = CString::new(runtime_config_path)?;
    let runtime_config_path = runtime_config_path.as_ptr();
    let initialize = self.init_symbol.deref();
    let close = self.close_symbol.deref();

    unsafe {
      let result = HResult::new(initialize(runtime_config_path, null_mut(), &mut handle));
      if result.is_failure() {
        close(handle);
        result.to_result()?;
      }
    }

    Ok(handle)
  }

  pub fn close(&self, host_context_handle: FxrContextHandle) -> HResult {
    unsafe { HResult::new(self.close_symbol.deref()(host_context_handle)) }
  }

  pub fn get_load_assembly_fn(
    &self,
    host_context_handle: FxrContextHandle,
  ) -> Result<LoadAssemblyAndGetFunctionPointer, Box<dyn Error>> {
    let mut load_assembly_fn = null_mut();
    let get_runtime_delegate = self.get_delegate_symbol.deref();
    let result = HResult::new(unsafe {
      get_runtime_delegate(
        host_context_handle,
        DelegateType::LoadAssemblyAndGetFunctionPointer,
        &mut load_assembly_fn,
      )
    })
    .to_result()?;

    Ok(unsafe { mem::transmute(load_assembly_fn) })
  }

  pub fn get_runtime_delegate(
    self,
    host_context_handle: FxrContextHandle,
    type_: DelegateType,
    delegate: *mut *mut c_void,
  ) -> HResult {
    unsafe {
      HResult::new(self.get_delegate_symbol.deref()(
        host_context_handle,
        type_,
        delegate,
      ))
    }
  }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DelegateType {
  ComActivation = 0,
  LoadInMemoryAssembly = 1,
  WinRTActivation = 2,
  ComRegister = 3,
  ComUnRegister = 4,
  LoadAssemblyAndGetFunctionPointer = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InitializeParameters {
  pub size: c_ulong,
  pub host_path: *const c_char,
  pub dotnet_root: *const c_char,
}

pub type FxrContextHandle = *mut c_void;

pub type MainFn = unsafe extern "C" fn(argc: i32, argv: *mut *const c_char) -> HRESULT;
pub type ErrorWriterFn = unsafe extern "C" fn(message: *const c_char);
pub type SetErrorWriterFn = unsafe extern "C" fn(error_writer: ErrorWriterFn) -> ErrorWriterFn;

pub type InitializeForDotnetCommandLineFn = unsafe extern "C" fn(
  argc: i32,
  argv: *mut *const c_char,
  parameters: *const InitializeParameters,
  host_context_handle: *mut FxrContextHandle,
) -> HRESULT;

pub type InitializeForRuntimeConfigFn = unsafe extern "C" fn(
  runtime_config_path: *const c_char,
  parameters: *const InitializeParameters,
  host_context_handle: *mut FxrContextHandle,
) -> HRESULT;

pub type GetRuntimePropertyValueFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  name: *const c_char,
  value: *mut *const c_char,
) -> HRESULT;

pub type SetRuntimePropertyValueFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  name: *const c_char,
  value: *const c_char,
) -> HRESULT;

pub type GetRuntimePropertiesFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  count: *mut c_ulong,
  keys: *mut *const c_char,
  values: *mut *const c_char,
) -> HRESULT;

pub type RunAppFn = unsafe extern "C" fn(host_context_handle: FxrContextHandle) -> HRESULT;
pub type GetRuntimeDelegateFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  type_: DelegateType,
  delegate: *mut *mut c_void,
) -> HRESULT;

pub type CloseFn = unsafe extern "C" fn(host_context_handle: FxrContextHandle) -> HRESULT;

// Signature of delegate returned by coreclr_delegate_type::load_assembly_and_get_function_pointer
pub type LoadAssemblyAndGetFunctionPointer = unsafe extern "C" fn(
  assembly_path: *const c_char,
  type_name: *const c_char,
  method_name: *const c_char,
  delegate_type_name: *const c_char,
  reserved: *const c_void,
  delegate: *mut *mut c_void,
) -> HRESULT;
