use crate::library::{FxrContextHandle, HostFxrLibrary, LoadAssemblyAndGetFunctionPointer};
use crate::nethost::get_hostfxr_path;
use std::error::Error;
use std::ffi::CString;
use std::mem;
use std::ptr::{null, null_mut};

pub struct HostFxr {
  handle: FxrContextHandle,
  library: HostFxrLibrary,
  create_delegate_fn: LoadAssemblyAndGetFunctionPointer,
}

impl HostFxr {
  pub fn new(runtime_config_path: &str) -> Result<Self, Box<dyn Error>> {
    let library = get_hostfxr_path()?;
    let library = HostFxrLibrary::new(library)?;
    let handle = library.initialize(runtime_config_path)?;
    let create_delegate_fn = library.get_load_assembly_fn(handle)?;

    Ok(Self {
      handle,
      library,
      create_delegate_fn,
    })
  }

  pub fn create_delegate<'a, T>(
    &'a self,
    assembly_path: &str,
    type_name: &str,
    method_name: &str,
    delegate_name: &str,
  ) -> Result<&'a T, Box<dyn Error>> {
    let assembly_path = CString::new(assembly_path)?.as_ptr();
    let type_name = CString::new(type_name)?.as_ptr();
    let method_name = CString::new(method_name)?.as_ptr();
    let delegate_name = CString::new(delegate_name)?.as_ptr();

    unsafe {
      let mut method = null_mut();
      let result = (self.create_delegate_fn)(
        assembly_path, // assembly_path: *const c_char,
        type_name,     // type_name: *const c_char,
        method_name,   // method_name: *const c_char,
        delegate_name, // delegate_type_name: *const c_char,
        null(),        // reserved: *const c_void,
        &mut method,   // delegate: *mut *mut c_void,
      );

      if result < 0 {
        bail!(
          "load_assembly_and_get_function_pointer failed with HRESULT {}",
          result
        );
      }

      Ok(&*(method as *const T))
    }
  }
}

impl Drop for HostFxr {
  fn drop(&mut self) {
    self.library.close(self.handle);
  }
}

#[cfg(test)]
mod tests {
  use crate::host::HostFxr;

  #[test]
  fn test_intialize() {
    let host = HostFxr::new("").unwrap();
  }
}
