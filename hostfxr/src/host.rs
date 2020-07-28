use crate::library::HostFxrLibrary;
use crate::types::{FxrContextHandle, LoadAssemblyAndGetFunctionPointer};
use libloading::Library;
use std::error::Error;
use std::ffi::{CString, OsStr};
use std::mem;
use std::ptr::{null, null_mut};

pub struct HostFxr {
  library: HostFxrLibrary,
  context: FxrContextHandle,
  load_assembly_fn: LoadAssemblyAndGetFunctionPointer,
}

impl HostFxr {
  pub fn new<P: AsRef<OsStr>, S: Into<String>>(
    filename: P,
    runtime_config_path: S,
  ) -> Result<Self, Box<dyn Error>> {
    let library = HostFxrLibrary::new(filename)?;
    let context = library.initialize(runtime_config_path)?;
    let load_assembly_fn = unsafe { library.get_load_assembly_fn(context)? };

    let mut result = Self {
      library,
      context,
      load_assembly_fn,
    };

    Ok(result)
  }

  pub unsafe fn load_assembly_and_get_function_pointer<T>(
    &self,
    assembly_path: String,
    type_name: String,
    method_name: String,
    // delegate_name: String,
  ) -> Result<&T, Box<dyn Error>> {
    let mut delegate = null_mut();

    let assembly_path = CString::new(assembly_path)?;
    let assembly_path = assembly_path.as_ptr();

    let type_name = CString::new(type_name)?;
    let type_name = type_name.as_ptr();

    let method_name = CString::new(method_name)?;
    let method_name = method_name.as_ptr();

    // let delegate_name = CString::new(delegate_name)?;
    // let delegate_name = delegate_name.as_ptr();

    let load_assembly_fn = self.load_assembly_fn;
    let result = load_assembly_fn(
      assembly_path,
      type_name,
      method_name,
      null_mut(),
      null_mut(),
      delegate,
    );

    // if result < 0 {
    //   bail!("Failed to load delegate {}", result);
    // }

    Ok(mem::transmute(delegate))
  }
}

impl Drop for HostFxr {
  fn drop(&mut self) {
    unsafe {
      self.library.close(self.context);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::host::HostFxr;
  use nethost::get_hostfxr_path;
  use std::error::Error;
  use std::os::raw::c_void;

  struct RuntimeMethods {
    get_type_handle: i32,
    get_method_handle: i32,
    get_assembly_handle: i32,
  }

  type Get = unsafe extern "C" fn(*mut c_void, i32) -> i32;

  #[test]
  fn test() -> Result<(), Box<dyn Error>> {
    let filename = get_hostfxr_path()?;
    let runtime_config_path = format!("{}/../../runtimeconfig.json", file!());
    let host = HostFxr::new(filename, runtime_config_path)?;
    let delegate: &Get = unsafe {
      host.load_assembly_and_get_function_pointer(
        "/Users/maarvin/Development/dotnet/runtime/bin/Debug/netcoreapp3.1/runtime.dll".to_string(),
        "Dotnet.Runtime".to_string(),
        "Entry".to_string(),
      )?
    };

    Ok(())
  }
}
