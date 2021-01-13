use std::convert::TryFrom;
use std::ffi::OsStr;

use hostfxr_sys::{char_t, hostfxr_initialize_parameters};

use crate::string::{IntoBytes, IntoPtr};
use crate::{
  error::{HostFxrError, HostFxrResult},
  parameters::HostFxrParameters,
};
use crate::{nethost::get_hostfxr_path, HostFxrContext};
use std::mem::{size_of, MaybeUninit};
use std::sync::Arc;

//
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};

#[derive(Debug, Clone)]
pub struct HostFxrLibrary {
  pub(crate) library: Arc<Library>,
  pub(crate) set_error_writer: Symbol<hostfxr_sys::hostfxr_set_error_writer_fn>,
  pub(crate) initialize_for_dotnet_command_line:
    Symbol<hostfxr_sys::hostfxr_initialize_for_dotnet_command_line_fn>,
  pub(crate) initialize_for_runtime_config:
    Symbol<hostfxr_sys::hostfxr_initialize_for_runtime_config_fn>,
  pub(crate) get_runtime_property_value:
    Symbol<hostfxr_sys::hostfxr_get_runtime_property_value_fn>,
  pub(crate) set_runtime_property_value:
    Symbol<hostfxr_sys::hostfxr_set_runtime_property_value_fn>,
  pub(crate) get_runtime_properties:
    Symbol<hostfxr_sys::hostfxr_get_runtime_properties_fn>,
  pub(crate) run_app: Symbol<hostfxr_sys::hostfxr_run_app_fn>,
  pub(crate) get_runtime_delegate: Symbol<hostfxr_sys::hostfxr_get_runtime_delegate_fn>,
  pub(crate) close: Symbol<hostfxr_sys::hostfxr_close_fn>,
}

impl HostFxrLibrary {
  pub fn new() -> HostFxrResult<HostFxrLibrary> {
    Self::from_path(get_hostfxr_path()?)
  }

  pub fn from_path<P: AsRef<OsStr>>(path: P) -> HostFxrResult<HostFxrLibrary> {
    Self::try_from(Arc::new(Library::new(path)?))
  }

  pub fn initialize_command_line<A, I>(
    &self,
    args: A,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<HostFxrContext>
  where
    A: IntoIterator<Item = I>,
    I: IntoBytes<char_t>,
  {
    let initialize_for_dotnet_command_line =
      self.initialize_for_dotnet_command_line.clone();
    let initialize_for_dotnet_command_line =
      initialize_for_dotnet_command_line.lift_option().unwrap();

    let (argc, argv) = {
      let mut vec: Vec<*const _> = args
        .into_iter()
        .map(|item| unsafe { item.into_ptr() })
        .collect();
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
    HostFxrContext::new(unsafe { handle.assume_init() }, self.clone())
  }

  pub fn initialize_runtime_config<R>(
    &self,
    config: R,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<HostFxrContext>
  where
    R: IntoBytes<char_t>,
  {
    let initialize_for_runtime_config = self.initialize_for_runtime_config.clone();
    let initialize_for_runtime_config =
      initialize_for_runtime_config.lift_option().unwrap();

    let runtime_config = config.into_bytes();
    let runtime_config = runtime_config.as_ptr();
    let mut handle = MaybeUninit::zeroed();
    let parameters = parameters.map(
      |HostFxrParameters {
         host_path,
         dotnet_root,
       }| { (host_path.into_bytes(), dotnet_root.into_bytes()) },
    );

    let parameters = match parameters {
      Some((host_path, dotnet_root)) => MaybeUninit::new(hostfxr_initialize_parameters {
        size: size_of::<hostfxr_initialize_parameters>() as u64,
        host_path: host_path.as_ptr(),
        dotnet_root: dotnet_root.as_ptr(),
      }),
      None => MaybeUninit::zeroed(),
    };

    let flag = unsafe {
      initialize_for_runtime_config(
        runtime_config,
        parameters.as_ptr(),
        handle.as_mut_ptr(),
      )
    };

    HostFxrError::from_status(flag)?;
    HostFxrContext::new(unsafe { handle.assume_init() }, self.clone())
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
