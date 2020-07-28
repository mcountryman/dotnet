use std::os::raw::{c_char, c_ulong, c_void};

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

pub type MainFn = unsafe extern "C" fn(argc: i32, argv: *mut *const c_char) -> i32;
pub type ErrorWriterFn = unsafe extern "C" fn(message: *const c_char);
pub type SetErrorWriterFn = unsafe extern "C" fn(error_writer: ErrorWriterFn) -> ErrorWriterFn;

pub type InitializeForDotnetCommandLineFn = unsafe extern "C" fn(
  argc: i32,
  argv: *mut *const c_char,
  parameters: *const InitializeParameters,
  host_context_handle: *mut FxrContextHandle,
) -> i32;

pub type InitializeForRuntimeConfigFn = unsafe extern "C" fn(
  runtime_config_path: *const c_char,
  parameters: *const InitializeParameters,
  host_context_handle: *mut FxrContextHandle,
) -> i32;

pub type GetRuntimePropertyValueFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  name: *const c_char,
  value: *mut *const c_char,
) -> i32;

pub type SetRuntimePropertyValueFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  name: *const c_char,
  value: *const c_char,
) -> i32;

pub type GetRuntimePropertiesFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  count: *mut c_ulong,
  keys: *mut *const c_char,
  values: *mut *const c_char,
) -> i32;

pub type RunAppFn = unsafe extern "C" fn(host_context_handle: FxrContextHandle) -> i32;
pub type GetRuntimeDelegateFn = unsafe extern "C" fn(
  host_context_handle: FxrContextHandle,
  type_: DelegateType,
  delegate: *mut *mut c_void,
) -> i32;

pub type CloseFn = unsafe extern "C" fn(host_context_handle: FxrContextHandle) -> i32;

// Signature of delegate returned by coreclr_delegate_type::load_assembly_and_get_function_pointer
pub type LoadAssemblyAndGetFunctionPointer = unsafe extern "C" fn(
  assembly_path: *const c_char,
  type_name: *const c_char,
  method_name: *const c_char,
  delegate_type_name: *const c_char,
  reserved: *const c_void,
  delegate: *mut *mut c_void,
) -> i32;
