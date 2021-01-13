use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsStr;

///
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};

use crate::error::{HostFxrError, HostFxrResult};
use crate::nethost::get_hostfxr_path;
use crate::parameters::HostFxrParameters;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct HostFxrLibrary {
  library: Library,
  set_error_writer: Symbol<hostfxr_sys::hostfxr_set_error_writer_fn>,
  initialize_for_dotnet_command_line:
    Symbol<hostfxr_sys::hostfxr_initialize_for_dotnet_command_line_fn>,
  initialize_for_runtime_config:
    Symbol<hostfxr_sys::hostfxr_initialize_for_runtime_config_fn>,
  get_runtime_property_value: Symbol<hostfxr_sys::hostfxr_get_runtime_property_value_fn>,
  set_runtime_property_value: Symbol<hostfxr_sys::hostfxr_set_runtime_property_value_fn>,
  get_runtime_properties: Symbol<hostfxr_sys::hostfxr_get_runtime_properties_fn>,
  run_app: Symbol<hostfxr_sys::hostfxr_run_app_fn>,
  get_runtime_delegate: Symbol<hostfxr_sys::hostfxr_get_runtime_delegate_fn>,
  close: Symbol<hostfxr_sys::hostfxr_close_fn>,
}

impl HostFxrLibrary {
  pub fn new() -> HostFxrResult<HostFxrLibrary> {
    Self::from_path(get_hostfxr_path()?)
  }

  pub fn from_path<P: AsRef<OsStr>>(path: P) -> HostFxrResult<HostFxrLibrary> {
    Self::try_from(Library::new(path)?)
  }

  /// Initializes the hosting components for a dotnet command line running an application
  ///
  /// # Arguments
  /// * `args` - Command line arguments
  /// * `parameters` - Optional runtime parameters
  ///
  /// This function parses the specified command-line arguments to determine the application to run. It will
  /// then find the corresponding .runtimeconfig.json and .deps.json with which to resolve frameworks and
  /// dependencies and prepare everything needed to load the runtime.
  ///
  /// This function only supports arguments for running an application. It does not support SDK commands.
  ///
  /// This function does not load the runtime.
  ///
  pub fn initialize_command_line<'a, A, I>(
    args: A,
    parameters: Option<HostFxrParameters>,
  ) -> i32
  where
    A: IntoIterator<Item = I>,
    I: Into<Cow<'a, str>>,
  {
    let parameters = parameters.into();

    unimplemented!()
  }

  pub fn initialize_runtime_config<'a, C>(
    runtime_config_path: C,
    parameters: Option<HostFxrParameters>,
  ) -> i32
  where
    C: Into<Cow<'a, str>>,
  {
    unimplemented!()
  }
}

impl TryFrom<Library> for HostFxrLibrary {
  type Error = HostFxrError;

  fn try_from(library: Library) -> Result<Self, Self::Error> {
    unsafe {
      let set_error_writer = library.get(b"hostfxr_set_error_writer")?;
      let initialize_for_dotnet_command_line =
        library.get(b"hostfxr_initialize_for_dotnet_command_line")?;
      let initialize_for_runtime_config =
        library.get(b"hostfxr_initialize_for_runtime_config")?;
      let get_runtime_property_value =
        library.get(b"hostfxr_get_runtime_property_value")?;
      let set_runtime_property_value =
        library.get(b"hostfxr_set_runtime_property_value")?;
      let get_runtime_properties = library.get(b"hostfxr_get_runtime_properties")?;
      let run_app = library.get(b"hostfxr_run_app")?;
      let get_runtime_delegate = library.get(b"hostfxr_get_runtime_delegate")?;
      let close = library.get(b"hostfxr_close")?;

      Ok(Self {
        library,
        set_error_writer,
        initialize_for_dotnet_command_line,
        initialize_for_runtime_config,
        get_runtime_property_value,
        set_runtime_property_value,
        get_runtime_properties,
        run_app,
        get_runtime_delegate,
        close,
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::HostFxrLibrary;

  #[test]
  fn test_hostfxr_new() {
    HostFxrLibrary::new().unwrap();
  }
}
