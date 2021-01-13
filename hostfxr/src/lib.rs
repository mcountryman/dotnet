use std::mem::MaybeUninit;
use std::{
  collections::HashMap,
  ptr::{null, null_mut},
};

use hostfxr_sys::{char_t, hostfxr_delegate_type, hostfxr_handle};
pub use library::HostFxrLibrary;

use crate::error::{HostFxrError, HostFxrResult};
use crate::string::{IntoBytes, IntoPtr, IntoString};

pub mod error;
mod library;
mod nethost;
mod parameters;
mod string;

type LoadAssemblyAndGetFunctionPointerFn = unsafe extern "C" fn(
  assembly_path: *const char_t,
  type_name: *const char_t,
  method_name: *const char_t,
  delegate_type_name: *const char_t,
  reserved: *mut ::std::os::raw::c_void,
  delegate: *mut *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int;

#[derive(Debug)]
pub struct HostFxrContext {
  handle: hostfxr_handle,
  library: HostFxrLibrary,

  get_function_pointer_fn: hostfxr_sys::get_function_pointer_fn,
  load_assembly_and_get_function_pointer: LoadAssemblyAndGetFunctionPointerFn,
}

impl HostFxrContext {
  pub fn new(handle: hostfxr_handle, library: HostFxrLibrary) -> HostFxrResult<Self> {
    let get_function_pointer_fn = Self::get_runtime_delegate(
      handle,
      &library,
      hostfxr_sys::hostfxr_delegate_type_hdt_get_function_pointer,
    )?;

    let load_assembly_and_get_function_pointer = Self::get_runtime_delegate(
      handle,
      &library,
      hostfxr_sys::hostfxr_delegate_type_hdt_load_assembly_and_get_function_pointer,
    )?;

    Ok(Self {
      handle,
      library,
      get_function_pointer_fn,
      load_assembly_and_get_function_pointer,
    })
  }

  /// Set the runtime property value name
  ///
  /// # Arguments
  /// * `name` - The runtime property name
  /// * `value` - The runtime property value
  ///
  /// # Example
  /// ```
  /// use std::path::Path;
  /// use std::error::Error;
  /// use hostfxr::HostFxrContext;
  ///
  /// fn add_probing_directory<P: AsRef<str>>(
  ///   ctx: HostFxrContext,
  ///   path: P,
  /// ) -> Result<(), Box<dyn Error>> {
  ///   const NAME: &str = "PROBING_DIRECTORIES";
  ///
  ///   let directories = ctx.get_runtime_property(NAME)?;
  ///   let directories = directories
  ///     .split(";")
  ///     .chain(Some(path.as_ref()))
  ///     .fold(String::new(), |a, b| a + b + ";");
  ///
  ///   ctx.set_runtime_property_value(NAME, directories)?;
  ///
  ///   Ok(())
  /// }
  /// ```
  pub fn set_runtime_property_value<N, V>(&self, name: N, value: V) -> HostFxrResult<()>
  where
    N: IntoBytes<char_t>,
    V: IntoBytes<char_t>,
  {
    let set_runtime_property_value = self.library.set_runtime_property_value.clone();
    let set_runtime_property_value = set_runtime_property_value.lift_option().unwrap();

    let name = name.into_bytes();
    let name = name.as_ptr();

    let value = value.into_bytes();
    let value = value.as_ptr();

    let flag = unsafe { set_runtime_property_value(self.handle, name, value) };
    HostFxrError::from_status(flag)?;

    Ok(())
  }

  /// Gets the runtime property value by name
  ///
  /// # Arguments
  /// * `name` - The name of the runtime property
  ///
  /// # Example
  /// ```
  /// use hostfxr::HostFxrContext;
  ///
  /// fn dump_property(ctx: HostFxrContext) {
  ///    println!(
  ///       "`RUNTIME_IDENTIFIER` = `{}`",
  ///       ctx.get_runtime_property("RUNTIME_IDENTIFIER").unwrap()
  ///     );
  ///    // `RUNTIME_IDENTIFIER` = `win10-x64`
  /// }
  /// ```
  pub fn get_runtime_property<N>(&self, name: N) -> HostFxrResult<String>
  where
    N: IntoBytes<char_t>,
  {
    let get_runtime_property_value = self.library.get_runtime_property_value.clone();
    let get_runtime_property_value = get_runtime_property_value.lift_option().unwrap();

    let name = name.into_bytes();
    let name = name.as_ptr();
    let mut value: *const char_t = null();

    let flag = unsafe {
      get_runtime_property_value(self.handle, name, &mut value as *mut *const _)
    };

    HostFxrError::from_status(flag)?;

    Ok(value.into_string())
  }

  /// Get all the runtime properties
  ///
  /// # Example
  /// ```
  /// use hostfxr::HostFxrContext;
  ///
  /// fn dump_properties(ctx: HostFxrContext) {
  ///   for (name, value) in ctx.get_runtime_properties().unwrap() {
  ///     println!("`{}` = `{}`", name, value);
  ///   }
  /// }
  /// ```
  pub fn get_runtime_properties(&self) -> HostFxrResult<HashMap<String, String>> {
    let get_runtime_properties = self.library.get_runtime_properties.clone();
    let get_runtime_properties = get_runtime_properties.lift_option().unwrap();

    let mut properties = HashMap::new();

    let mut count: u64 = 2048;
    let mut keys = vec![null(); count as usize];
    let mut values = vec![null(); count as usize];

    let flag = unsafe {
      get_runtime_properties(
        self.handle,
        &mut count as *mut _,
        keys.as_mut_ptr(),
        values.as_mut_ptr(),
      )
    };

    HostFxrError::from_status(flag)?;

    keys
      .into_iter()
      .zip(values)
      .take(count as usize)
      .for_each(|(key, value)| {
        let key = key.into_string();
        let value = value.into_string();

        properties.insert(key, value);
      });

    Ok(properties)
  }

  /// Load CoreCLR and run the application for an initialized host context
  ///
  /// The host_context_handle must have been initialized using
  /// hostfxr_initialize_for_dotnet_command_line.
  ///
  /// # Example
  /// ```
  ///
  /// use hostfxr::HostFxrLibrary;
  ///
  /// fn run_app<A: AsRef<str>>(exe_path: A) {
  ///   let hostfxr = HostFxrLibrary::new().expect("Failed to load hostfxr");
  ///   let hostfxr = hostfxr
  ///     .initialize_command_line(
  ///       &[exe_path.as_ref()],
  ///       None,
  ///     )
  ///     .expect("Failed to initialize hostfxr");
  ///
  ///   hostfxr.run_app().expect(&format!("Failed to run app `{}`", exe_path.as_ref()));
  /// }
  ///
  /// ```
  pub fn run_app(&self) -> HostFxrResult<()> {
    let run_app = self.library.run_app.clone();
    let run_app = run_app.lift_option().unwrap();

    let flag = unsafe { run_app(self.handle) };

    HostFxrError::from_status(flag)
  }

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
  ///
  /// # Example
  /// ```
  /// use hostfxr::HostFxrContext;
  ///
  /// type AddFn = extern "C" fn(a: i32, b: i32) -> i32;
  ///
  /// fn get_add_fn(ctx: HostFxrContext) -> AddFn {
  ///   ctx.load_assembly_and_get_delegate(
  ///     std::fs::canonicalize("../bridge/bin/Debug/net5.0/bridge.dll")
  ///       .unwrap()
  ///       .to_str()
  ///       .unwrap(),
  ///     "Methods, add, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
  ///     "Add",
  ///     "Methods+AddFn, add, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
  ///   ).expect("Failed to resolve Add method")
  /// }
  /// ```
  pub fn load_assembly_and_get_delegate<F, A, T, M, D>(
    &self,
    assembly_path: A,
    type_name: T,
    method_name: M,
    delegate_type_name: D,
  ) -> HostFxrResult<F>
  where
    A: IntoBytes<char_t>,
    T: IntoBytes<char_t>,
    M: IntoBytes<char_t>,
    D: IntoBytes<char_t>,
  {
    let native = self.load_assembly_and_get_function_pointer;
    let mut delegate = MaybeUninit::<F>::zeroed();
    let delegate_ptr = delegate.as_mut_ptr() as *mut _ as *mut *mut _;

    let flag = unsafe {
      native(
        assembly_path.into_ptr(),
        type_name.into_ptr(),
        method_name.into_ptr(),
        delegate_type_name.into_ptr(),
        null_mut(),
        delegate_ptr,
      )
    };

    HostFxrError::from_status(flag)?;

    Ok(unsafe { delegate.assume_init() })
  }

  fn get_runtime_delegate<F>(
    handle: hostfxr_handle,
    library: &HostFxrLibrary,
    kind: hostfxr_delegate_type,
  ) -> HostFxrResult<F> {
    let get_runtime_delegate = library.get_runtime_delegate.clone();
    let get_runtime_delegate = get_runtime_delegate.lift_option().unwrap();

    let mut delegate = MaybeUninit::<F>::uninit();
    let delegate_ptr = delegate.as_mut_ptr() as *mut _ as *mut *mut _;
    let flag = unsafe { get_runtime_delegate(handle, kind, delegate_ptr) };

    HostFxrError::from_status(flag)?;

    Ok(unsafe { delegate.assume_init() })
  }
}

impl Drop for HostFxrContext {
  fn drop(&mut self) {
    let close = self.library.close.clone();
    let close = close.lift_option().unwrap();

    unsafe { close(self.handle) };
  }
}

#[cfg(test)]
mod tests {
  use crate::HostFxrLibrary;

  type InitializeFn = extern "C" fn();

  #[test]
  fn test_hostfxr_new() {
    let hostfxr = HostFxrLibrary::new().unwrap();
    let hostfxr = hostfxr
      // .initialize_command_line(
      //   &[
      //     std::fs::canonicalize("../bridge/bin/Debug/net5.0/bridge.dll")
      //       .unwrap()
      //       .to_str()
      //       .unwrap(),
      //     "Hello There",
      //     "CSharp",
      //   ],
      //   None,
      // )
      .initialize_runtime_config(
        std::fs::canonicalize("../bridge/bridge.runtimeconfig.json")
          .unwrap()
          .to_str()
          .unwrap(),
        None,
      )
      .unwrap();

    hostfxr
      .get_runtime_properties()
      .unwrap()
      .into_iter()
      .for_each(|(name, value)| {
        println!("`{}` = `{}`", name, value);
      });

    let initialize: InitializeFn = hostfxr
      .load_assembly_and_get_delegate(
        std::fs::canonicalize("../bridge/bin/Debug/net5.0/bridge.dll")
          .unwrap()
          .to_str()
          .unwrap(),
        "Bridge, bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
        "Initialize",
        "Bridge+InitializeFn, bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
      )
      .unwrap();

    initialize();
  }
}
