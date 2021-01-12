use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::ffi::{c_void, OsStr};
use std::ptr::{null, null_mut};

//
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};

use hostfxr_sys::{
  char_t, hostfxr_delegate_type, hostfxr_handle, hostfxr_initialize_parameters,
};

use crate::error::{HostFxrError, HostFxrResult};
use crate::nethost::get_hostfxr_path;
use crate::string::{IntoBytes, IntoPtr, IntoString};
use std::collections::HashMap;
use std::mem::{size_of, MaybeUninit};
use std::sync::Arc;

pub mod error;
mod nethost;
#[macro_use]
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

  /// Get all the runtime properties for the host context.
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

  pub fn run_app(&self) -> HostFxrResult<()> {
    let run_app = self.library.run_app.clone();
    let run_app = run_app.lift_option().unwrap();

    let flag = unsafe { run_app(self.handle) };

    HostFxrError::from_status(flag)
  }

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
    let mut delegate_ptr = delegate.as_mut_ptr() as *mut c_void;
    let delegate_ptr = &mut delegate_ptr as *mut *mut _;

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

#[derive(Debug, Clone)]
pub struct HostFxrParameters<'a> {
  pub host_path: Cow<'a, str>,
  pub dotnet_root: Cow<'a, str>,
}

#[derive(Debug, Clone)]
pub struct HostFxrLibrary {
  library: Arc<Library>,
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
        std::fs::canonicalize("../bridge/bin/Debug/net5.0/bridge.runtimeconfig.2.json")
          .unwrap()
          .to_str()
          .unwrap(),
        None,
      )
      .unwrap();

    let initialize: InitializeFn = hostfxr
      .load_assembly_and_get_delegate(
        std::fs::canonicalize("../bridge/bin/Debug/net5.0/bridge.dll")
          .unwrap()
          .to_str()
          .unwrap(),
        "Bridge",
        "Initialize",
        "Bridge.InitializeFn",
      )
      .unwrap();

    initialize();
  }
}
