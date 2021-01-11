#[macro_use]
extern crate lazy_static;

use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::ptr::null;

//
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};

use hostfxr_sys::{hostfxr_handle, hostfxr_initialize_parameters};

use crate::error::{HostFxrError, HostFxrResult};
use crate::nethost::get_hostfxr_path;
use crate::string::IntoBytes;
use std::mem::{size_of, MaybeUninit};

pub mod error;
mod nethost;
#[macro_use]
mod string;

#[derive(Debug)]
pub struct HostFxrHandle {
  handle: hostfxr_handle,
  library: HostFxrLibrary,
}

#[derive(Debug, Clone)]
pub struct HostFxrParameters<'a> {
  pub host_path: Cow<'a, str>,
  pub dotnet_root: Cow<'a, str>,
}

#[derive(Debug, Copy, Clone)]
pub enum HostFxrDelegateKind {
  ComActivation,
  LoadInMemoryAssembly,
  WinrtActivation,
  ComRegister,
  ComUnregister,
  LoadAssemblyAndGetFunctionPointer,
  GetFunctionPointer,
}

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

  pub fn initialize_command_line<A, I>(
    self,
    args: A,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<HostFxrHandle>
  where
    A: IntoIterator<Item = I>,
    I: IntoBytes<hostfxr_sys::char_t>,
  {
    let initialize_for_dotnet_command_line =
      self.initialize_for_dotnet_command_line.clone();
    let initialize_for_dotnet_command_line =
      initialize_for_dotnet_command_line.lift_option().unwrap();

    let (argv, argc) = into_args!(args);
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
      None => MaybeUninit::uninit(),
    };

    let flag = unsafe {
      initialize_for_dotnet_command_line(
        argc,
        argv,
        parameters.as_ptr(),
        handle.as_mut_ptr(),
      )
    };

    if flag >> 31 != 0 {
      #[cfg(windows)]
      Err(HostFxrError::Io(std::io::Error::from_raw_os_error(flag)))
    } else {
      Ok(HostFxrHandle {
        handle: unsafe { handle.assume_init() },
        library: self,
      })
    }
  }

  pub fn initialize_runtime_config<R>(
    self,
    config: R,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<HostFxrHandle>
  where
    R: IntoBytes<hostfxr_sys::char_t>,
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

    if flag >> 31 != 0 {
      #[cfg(windows)]
      Err(HostFxrError::Io(std::io::Error::from_raw_os_error(flag)))
    } else {
      Ok(HostFxrHandle {
        handle: unsafe { handle.assume_init() },
        library: self,
      })
    }
  }

  //
  // pub fn get_runtime_property_value<N>(
  //   &self,
  //   handle: HostFxrHandle,
  //   name: N,
  // ) -> HostFxrResult<String>
  // where
  //   N: IntoOsString,
  // {
  //   unimplemented!()
  // }
  //
  // pub fn set_runtime_property_value<N, V>(
  //   &self,
  //   handle: HostFxrHandle,
  //   name: N,
  //   value: V,
  // ) -> HostFxrResult<()>
  // where
  //   N: IntoOsString,
  //   V: IntoOsString,
  // {
  //   unimplemented!()
  // }
  //
  // pub fn get_runtime_properties(
  //   &self,
  //   handle: HostFxrHandle,
  // ) -> HostFxrResult<HashMap<String, String>> {
  //   unimplemented!()
  // }
  //
  // pub fn run_app(&self, handle: HostFxrHandle) -> HostFxrResult<()> {
  //   unimplemented!()
  // }
  //
  // pub fn get_runtime_delegate(
  //   &self,
  //   handle: HostFxrHandle,
  //   kind: HostFxrDelegateKind,
  // ) -> HostFxrResult<()> {
  //   unimplemented!()
  // }
  //
  // pub fn close(&self, handle: HostFxrHandle) -> HostFxrResult<()> {
  //   unimplemented!()
  // }
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
  use crate::{HostFxrLibrary, HostFxrParameters};
  use std::borrow::Cow;

  #[test]
  fn test_hostfxr_new() {
    let hostfxr = HostFxrLibrary::new().unwrap();
    let hostfxr = hostfxr.initialize_runtime_config("", None).unwrap();
  }
}
