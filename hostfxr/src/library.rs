use std::convert::TryFrom;
use std::ffi::OsStr;

use dotnet_hostfxr_sys::char_t;

use crate::string::{IntoBytes, IntoPtr};
use crate::{
  error::{HostFxrError, HostFxrResult},
  parameters::HostFxrParameters,
};
use crate::{nethost::get_hostfxr_path, HostFxr};
use std::mem::MaybeUninit;
use std::sync::Arc;

//
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};

/// Wrapper around hostfxr dynamic library exports.
#[derive(Debug, Clone)]
pub struct HostFxrLibrary {
  pub(crate) library: Arc<Library>,
  pub(crate) set_error_writer: Symbol<dotnet_hostfxr_sys::hostfxr_set_error_writer_fn>,
  pub(crate) initialize_for_dotnet_command_line:
    Symbol<dotnet_hostfxr_sys::hostfxr_initialize_for_dotnet_command_line_fn>,
  pub(crate) initialize_for_runtime_config:
    Symbol<dotnet_hostfxr_sys::hostfxr_initialize_for_runtime_config_fn>,
  pub(crate) get_runtime_property_value:
    Symbol<dotnet_hostfxr_sys::hostfxr_get_runtime_property_value_fn>,
  pub(crate) set_runtime_property_value:
    Symbol<dotnet_hostfxr_sys::hostfxr_set_runtime_property_value_fn>,
  pub(crate) get_runtime_properties:
    Symbol<dotnet_hostfxr_sys::hostfxr_get_runtime_properties_fn>,
  pub(crate) run_app: Symbol<dotnet_hostfxr_sys::hostfxr_run_app_fn>,
  pub(crate) get_runtime_delegate:
    Symbol<dotnet_hostfxr_sys::hostfxr_get_runtime_delegate_fn>,
  pub(crate) close: Symbol<dotnet_hostfxr_sys::hostfxr_close_fn>,
}

impl HostFxrLibrary {
  /// Detect and load hostfxr library from environment
  pub fn new() -> HostFxrResult<HostFxrLibrary> {
    Self::from_path(get_hostfxr_path()?)
  }

  /// Load hostfxr library from supplied path
  ///
  /// # Arguments
  /// * `path` - Path to hostfxr library on system.
  pub fn from_path<P: AsRef<OsStr>>(path: P) -> HostFxrResult<HostFxrLibrary> {
    Self::try_from(Arc::new(Library::new(path)?))
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
    I: IntoBytes<char_t>,
  {
    let initialize_for_dotnet_command_line =
      self.initialize_for_dotnet_command_line.clone();
    let initialize_for_dotnet_command_line =
      initialize_for_dotnet_command_line.lift_option().unwrap();

    // Convert args to leaky ptr of ptrs
    let (argc, argv) = unsafe {
      let mut vec: Vec<*const _> = args.into_iter().map(|item| item.into_ptr()).collect();
      let len = vec.len() as _;
      let ptr = vec.as_mut_ptr();

      std::mem::forget(vec);

      (len, ptr)
    };

    let mut handle = MaybeUninit::zeroed();
    let flag = unsafe {
      initialize_for_dotnet_command_line(
        argc,
        argv,
        parameters.into_ptr(),
        handle.as_mut_ptr(),
      )
    };

    HostFxrError::from_status(flag)?;
    HostFxr::new(unsafe { handle.assume_init() }, self.clone())
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
    R: IntoBytes<char_t>,
  {
    let initialize_for_runtime_config = self.initialize_for_runtime_config.clone();
    let initialize_for_runtime_config =
      initialize_for_runtime_config.lift_option().unwrap();

    let mut handle = MaybeUninit::zeroed();

    let flag = unsafe {
      initialize_for_runtime_config(
        runtime_config.into_ptr(),
        parameters.into_ptr(),
        handle.as_mut_ptr(),
      )
    };

    HostFxrError::from_status(flag)?;
    HostFxr::new(unsafe { handle.assume_init() }, self.clone())
  }
}

impl TryFrom<Arc<Library>> for HostFxrLibrary {
  type Error = HostFxrError;

  fn try_from(library: Arc<Library>) -> Result<Self, Self::Error> {
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
