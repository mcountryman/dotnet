use delegate::{LoadAssemblyAndGetFunctionPointerDelegate, RuntimeDelegate};
use dotnet_hostfxr_sys::char_t;
use std::{
  collections::HashMap,
  ffi::c_void,
  ops::DerefMut,
  ptr::NonNull,
  sync::{Arc, Mutex},
};
use string::IntoFxrBytes;
use symbol::{
  CloseSymbol, GetRuntimeDelegateSymbol, GetRuntimePropertiesSymbol,
  GetRuntimePropertyValueSymbol, RunAppSymbol, SetRuntimePropertyValueSymbol,
};

pub mod delegate;
pub mod error;
pub mod symbol;
pub use error::*;

pub mod library;
pub use library::*;

mod nethost;
mod parameters;
mod string;

#[derive(Debug, Clone)]
pub struct HostFxr<'lib> {
  handle: Arc<Mutex<NonNull<c_void>>>,

  close: CloseSymbol<'lib>,
  run_app: RunAppSymbol<'lib>,
  get_runtime_delegate: GetRuntimeDelegateSymbol<'lib>,
  set_runtime_property: SetRuntimePropertyValueSymbol<'lib>,
  get_runtime_property: GetRuntimePropertyValueSymbol<'lib>,
  get_runtime_properties: GetRuntimePropertiesSymbol<'lib>,
  load_assembly_and_get_function_pointer: LoadAssemblyAndGetFunctionPointerDelegate,
}

impl<'lib> HostFxr<'lib> {
  pub fn new(
    mut handle: NonNull<c_void>,
    library: &HostFxrLibrary<'lib>,
  ) -> HostFxrResult<Self> {
    let load_assembly_and_get_function_pointer =
      library.get_runtime_delegate(&mut handle)?;

    Ok(Self {
      handle: Arc::new(Mutex::new(handle)),
      close: library.close.clone(),
      run_app: library.run_app.clone(),
      get_runtime_delegate: library.get_runtime_delegate.clone(),
      set_runtime_property: library.set_runtime_property.clone(),
      get_runtime_property: library.get_runtime_property.clone(),
      get_runtime_properties: library.get_runtime_properties.clone(),
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
  /// use dotnet_hostfxr::HostFxr;
  ///
  /// fn add_probing_directory<P: AsRef<str>>(
  ///   mut ctx: HostFxr,
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
    N: IntoFxrBytes<char_t>,
    V: IntoFxrBytes<char_t>,
  {
    let mut handle = self
      .handle
      .lock()
      .map_err(|_| HostFxrError::PoisonedHandle)?;

    self
      .set_runtime_property
      .invoke(handle.deref_mut(), name, value)
  }

  /// Gets the runtime property value by name
  ///
  /// # Arguments
  /// * `name` - The name of the runtime property
  ///
  /// # Example
  /// ```
  /// use dotnet_hostfxr::HostFxr;
  ///
  /// fn dump_property(mut ctx: HostFxr) {
  ///    println!(
  ///       "`RUNTIME_IDENTIFIER` = `{}`",
  ///       ctx.get_runtime_property("RUNTIME_IDENTIFIER").unwrap()
  ///     );
  ///    // `RUNTIME_IDENTIFIER` = `win10-x64`
  /// }
  /// ```
  pub fn get_runtime_property<N>(&self, name: N) -> HostFxrResult<String>
  where
    N: IntoFxrBytes<char_t>,
  {
    let mut handle = self
      .handle
      .lock()
      .map_err(|_| HostFxrError::PoisonedHandle)?;

    self.get_runtime_property.invoke(handle.deref_mut(), name)
  }

  /// Get all the runtime properties
  ///
  /// # Example
  /// ```
  /// use dotnet_hostfxr::HostFxr;
  ///
  /// fn dump_properties(mut ctx: HostFxr) {
  ///   for (name, value) in ctx.get_runtime_properties().unwrap() {
  ///     println!("`{}` = `{}`", name, value);
  ///   }
  /// }
  /// ```
  pub fn get_runtime_properties(&self) -> HostFxrResult<HashMap<String, String>> {
    let mut handle = self
      .handle
      .lock()
      .map_err(|_| HostFxrError::PoisonedHandle)?;

    self.get_runtime_properties.invoke(handle.deref_mut())
  }

  /// Load CoreCLR and run the application for an initialized host context
  ///
  /// The host_context_handle must have been initialized using
  /// hostfxr_initialize_for_dotnet_command_line.
  ///
  /// # Example
  /// ```
  ///
  /// use dotnet_hostfxr::HostFxrLibrary;
  ///
  /// fn run_app<A: AsRef<str>>(exe_path: A) {
  ///   let hostfxr = HostFxrLibrary::get().expect("Failed to initialize hostfxr");
  ///   let mut hostfxr = hostfxr
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
    let mut handle = self
      .handle
      .lock()
      .map_err(|_| HostFxrError::PoisonedHandle)?;

    self.run_app.invoke(handle.deref_mut())
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
  /// use dotnet_hostfxr::HostFxr;
  ///
  /// type AddFn = extern "C" fn(a: i32, b: i32) -> i32;
  ///
  /// fn get_add_fn(mut ctx: HostFxr) -> AddFn {
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
    A: IntoFxrBytes<char_t>,
    T: IntoFxrBytes<char_t>,
    M: IntoFxrBytes<char_t>,
    D: IntoFxrBytes<char_t>,
  {
    let guard = self.handle.lock().unwrap();
    let delegate = self.load_assembly_and_get_function_pointer.invoke(
      assembly_path,
      type_name,
      method_name,
      delegate_type_name,
    )?;

    // Attempt to keep rustc from optimizing away our lock
    std::mem::drop(guard);

    Ok(delegate)
  }

  pub fn get_runtime_delegate<D>(&self) -> HostFxrResult<D>
  where
    D: RuntimeDelegate,
  {
    let mut handle = self
      .handle
      .lock()
      .map_err(|_| HostFxrError::PoisonedHandle)?;

    Ok(D::from_native(
      self.get_runtime_delegate.invoke::<D>(handle.deref_mut())?,
    ))
  }
}

unsafe impl Send for HostFxr<'_> {}
unsafe impl Sync for HostFxr<'_> {}

impl Drop for HostFxr<'_> {
  fn drop(&mut self) {
    self
      .close
      .invoke(self.handle.lock().expect("Poisoned handle").deref_mut())
      .expect("Failed to close runtime handle");
  }
}
