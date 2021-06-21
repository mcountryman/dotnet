use dotnet_hostfxr_sys::{
  char_t, hostfxr_delegate_type,
  hostfxr_delegate_type_hdt_load_assembly_and_get_function_pointer,
};
use std::{ffi::c_void, mem::MaybeUninit, os::raw::c_int, ptr::null_mut};

use crate::{
  string::{IntoFxrBytes, IntoFxrPtr},
  HostFxrError, HostFxrResult,
};

pub trait RuntimeDelegate {
  type Fn;
  const KIND: hostfxr_delegate_type;

  fn from_native(native: Self::Fn) -> Self;
}

type LoadAssemblyAndGetFunctionPointerFn = unsafe extern "C" fn(
  assembly_path: *const char_t,
  type_name: *const char_t,
  method_name: *const char_t,
  delegate_type_name: *const char_t,
  reserved: *mut c_void,
  delegate: *mut *mut c_void,
) -> c_int;

#[derive(Debug, Clone, Copy)]
pub struct LoadAssemblyAndGetFunctionPointerDelegate(LoadAssemblyAndGetFunctionPointerFn);

impl LoadAssemblyAndGetFunctionPointerDelegate {
  /// Calling this function will load the specified assembly in isolation (into its own
  /// `AssemblyLoadContext`) and it will use `AssemblyDependencyResolver` on it to provide
  /// dependency resolution. Once loaded it will find the specified type and method and
  /// return a native function pointer to that method. The method's signature can be
  /// specified via the delegate type name.
  ///
  /// # Arguments
  /// * `assembly_path` - Path to the assembly to load. In case of complex component, this
  /// should be the main assembly of the component (the one with the `.deps.json` next to
  /// it). Note that this does not have to be the assembly from which the `type_name` and
  /// `method_name` are.
  ///  * `type_name` - Assembly qualified type name to find
  ///  * `method_name` - Name of the method on the `type_name` to find. The method must be
  ///  `static` and must match the signature of `delegate_type_name`.
  ///  * `delegate_type_name` - Assembly qualified delegate type name for the method
  /// signature, or null. If this is null, the method signature is assumed to be
  /// `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
  pub fn invoke<F, A, T, M, D>(
    &self,
    assembly_path: A,
    type_name: T,
    method_name: M,
    delegate_type_name: D,
  ) -> HostFxrResult<F>
  where
    A: IntoFxrBytes<char_t>,
    T: IntoFxrBytes<char_t>,
    M: IntoFxrBytes<char_t>,
    D: IntoFxrBytes<char_t>,
  {
    let mut delegate = MaybeUninit::<F>::zeroed();
    let delegate_ptr = delegate.as_mut_ptr() as *mut _ as *mut *mut _;

    let flag = unsafe {
      self.0(
        assembly_path.into_fxr_ptr(),
        type_name.into_fxr_ptr(),
        method_name.into_fxr_ptr(),
        delegate_type_name.into_fxr_ptr(),
        null_mut(),
        delegate_ptr,
      )
    };

    HostFxrError::from_status(flag)?;

    Ok(unsafe { delegate.assume_init() })
  }
}

impl RuntimeDelegate for LoadAssemblyAndGetFunctionPointerDelegate {
  type Fn = LoadAssemblyAndGetFunctionPointerFn;
  const KIND: hostfxr_delegate_type =
    hostfxr_delegate_type_hdt_load_assembly_and_get_function_pointer;

  fn from_native(native: Self::Fn) -> Self {
    Self(native)
  }
}
