use crate::{
  delegate::RuntimeDelegate,
  error::HostFxrResult,
  nethost::get_hostfxr_path,
  parameters::HostFxrParameters,
  string::IntoFxrBytes,
  symbol::{
    CloseSymbol, GetRuntimeDelegateSymbol, GetRuntimePropertiesSymbol,
    GetRuntimePropertyValueSymbol, InitializeCommandLineSymbol, InitializeConfigSymbol,
    RunAppSymbol, SetRuntimePropertyValueSymbol,
  },
  HostFxr, HostFxrError,
};
use dotnet_hostfxr_sys::char_t;
use libloading::Library;
use once_cell::sync::OnceCell;
use std::{ffi::c_void, ptr::NonNull};

static LIBRARY: OnceCell<Library> = OnceCell::new();
static CURRENT: OnceCell<HostFxrLibrary<'static>> = OnceCell::new();

/// Wrapper around hostfxr dynamic library exports.
#[derive(Debug, Clone)]
pub struct HostFxrLibrary<'lib> {
  pub close: CloseSymbol<'lib>,
  pub run_app: RunAppSymbol<'lib>,
  pub initialize_config: InitializeConfigSymbol<'lib>,
  pub initialize_command_line: InitializeCommandLineSymbol<'lib>,
  pub get_runtime_delegate: GetRuntimeDelegateSymbol<'lib>,
  pub set_runtime_property: SetRuntimePropertyValueSymbol<'lib>,
  pub get_runtime_property: GetRuntimePropertyValueSymbol<'lib>,
  pub get_runtime_properties: GetRuntimePropertiesSymbol<'lib>,
}

impl HostFxrLibrary<'static> {
  pub fn get() -> HostFxrResult<&'static Self> {
    if let Some(current) = CURRENT.get() {
      return Ok(current);
    }

    let library = Self::get_library()?;
    let library = HostFxrLibrary::from_library(library)?;

    Ok(match CURRENT.try_insert(library) {
      Ok(current) => current,
      Err((current, _)) => &current,
    })
  }

  fn get_library() -> HostFxrResult<&'static Library> {
    if let Some(library) = LIBRARY.get() {
      return Ok(library);
    }

    let library = get_hostfxr_path()?;
    let library = Library::new(&library)?;

    Ok(match LIBRARY.try_insert(library) {
      Ok(library) => library,
      Err((library, _)) => &library,
    })
  }
}

impl<'lib> HostFxrLibrary<'lib> {
  pub fn from_library(library: &Library) -> HostFxrResult<HostFxrLibrary<'_>> {
    Ok(HostFxrLibrary {
      close: CloseSymbol::new(library)?,
      run_app: RunAppSymbol::new(library)?,
      initialize_config: InitializeConfigSymbol::new(library)?,
      initialize_command_line: InitializeCommandLineSymbol::new(library)?,
      get_runtime_delegate: GetRuntimeDelegateSymbol::new(library)?,
      set_runtime_property: SetRuntimePropertyValueSymbol::new(library)?,
      get_runtime_property: GetRuntimePropertyValueSymbol::new(library)?,
      get_runtime_properties: GetRuntimePropertiesSymbol::new(library)?,
    })
  }

  /// Initializes the hosting components for a dotnet command line running an application
  ///
  /// This function parses the specified command-line arguments to determine the application to run. It will
  /// then find the corresponding .runtimeconfig.json and .deps.json with which to resolve frameworks and
  /// dependencies and prepare everything needed to load the runtime.
  ///
  /// This function only supports arguments for running an application. It does not support SDK commands.
  ///
  /// This function does not load the runtime.
  ///
  /// # Arguments
  /// * `args` - Command-line arguments for running an application
  /// * `parameters` - Additional parameters for initialization
  pub fn initialize_command_line<A, I>(
    &self,
    args: A,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<HostFxr>
  where
    A: IntoIterator<Item = I>,
    I: IntoFxrBytes<char_t>,
  {
    let handle = self.initialize_command_line.invoke(args, parameters)?;
    match handle {
      Some(handle) => HostFxr::new(handle, self),
      None => Err(HostFxrError::BadHandle),
    }
  }

  /// Initializes the hosting components using a .runtimeconfig.json file
  ///
  /// This function will process the .runtimeconfig.json to resolve frameworks and prepare everything needed
  /// to load the runtime. It will only process the .deps.json from frameworks (not any app/component that
  /// may be next to the .runtimeconfig.json).
  ///
  /// This function does not load the runtime.
  ///
  /// If called when the runtime has already been loaded, this function will check if the specified runtime
  /// config is compatible with the existing runtime.
  ///
  /// # Arguments
  /// * `runtime_config` - Path to the .runtimeconfig.json file
  /// * `parameters` - Additional parameters for intialization
  pub fn initialize_runtime_config<R>(
    &self,
    runtime_config: R,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<HostFxr>
  where
    R: IntoFxrBytes<char_t>,
  {
    let handle = self.initialize_config.invoke(runtime_config, parameters)?;
    match handle {
      Some(handle) => HostFxr::new(handle, self),
      None => Err(HostFxrError::BadHandle),
    }
  }

  pub fn get_runtime_delegate<D>(&self, handle: &mut NonNull<c_void>) -> HostFxrResult<D>
  where
    D: RuntimeDelegate,
  {
    Ok(D::from_native(
      self.get_runtime_delegate.invoke::<D>(handle)?,
    ))
  }
}
