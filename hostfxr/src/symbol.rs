use crate::{
  delegate::RuntimeDelegate,
  parameters::HostFxrParameters,
  string::{IntoFxrBytes, IntoFxrPtr, IntoFxrString},
  HostFxrError, HostFxrResult,
};
use dotnet_hostfxr_sys::char_t;
use libloading::{Library, Symbol};
use std::{
  collections::HashMap,
  ffi::c_void,
  mem::{ManuallyDrop, MaybeUninit},
  ptr::{null, null_mut, NonNull},
};

/// Safely wraps `hostfxr_initialize_for_dotnet_command_line` calls
#[derive(Debug, Clone)]
pub struct InitializeCommandLineSymbol<'lib>(
  Symbol<'lib, dotnet_hostfxr_sys::hostfxr_initialize_for_dotnet_command_line_fn>,
);

impl<'lib> InitializeCommandLineSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe {
      library.get(b"hostfxr_initialize_for_dotnet_command_line")?
    }))
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
  pub fn invoke<A, I>(
    &self,
    args: A,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<Option<NonNull<c_void>>>
  where
    A: IntoIterator<Item = I>,
    I: IntoFxrBytes<char_t>,
  {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_initialize_for_dotnet_command_line` not found");

    // Copy each argument into a HostFxr API compatible string, leak created string, collect
    // into a vector that is leaked as well.
    let (argc, argv) = {
      let vec: Vec<*const _> = args
        .into_iter()
        .map(|item| ManuallyDrop::new(item.into_fxr_bytes()).as_ptr())
        .collect();
      let mut vec = ManuallyDrop::new(vec);
      let len = vec.len() as _;
      let ptr = vec.as_mut_ptr();

      (len, ptr)
    };

    let mut handle = null_mut();
    let flag = unsafe { symbol(argc, argv, parameters.into_fxr_ptr(), &mut handle) };

    HostFxrError::from_status(flag)?;

    Ok(NonNull::new(handle))
  }
}

/// Safely wraps `hostfxr_initialize_for_runtime_config` calls
#[derive(Debug, Clone)]
pub struct InitializeConfigSymbol<'lib>(
  Symbol<'lib, dotnet_hostfxr_sys::hostfxr_initialize_for_runtime_config_fn>,
);

impl<'lib> InitializeConfigSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe {
      library.get(b"hostfxr_initialize_for_runtime_config")?
    }))
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
  pub fn invoke<R>(
    &self,
    runtime_config: R,
    parameters: Option<HostFxrParameters>,
  ) -> HostFxrResult<Option<NonNull<c_void>>>
  where
    R: IntoFxrBytes<char_t>,
  {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_initialize_for_runtime_config` not found");

    let mut handle = null_mut();
    let flag = unsafe {
      symbol(
        runtime_config.into_fxr_ptr(),
        parameters.into_fxr_ptr(),
        &mut handle,
      )
    };

    HostFxrError::from_status(flag)?;

    Ok(NonNull::new(handle))
  }
}

/// Safely wraps `hostfxr_set_runtime_property_value` calls
#[derive(Debug, Clone)]
pub struct SetRuntimePropertyValueSymbol<'lib>(
  Symbol<'lib, dotnet_hostfxr_sys::hostfxr_set_runtime_property_value_fn>,
);

impl<'lib> SetRuntimePropertyValueSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe {
      library.get(b"hostfxr_set_runtime_property_value")?
    }))
  }

  /// Set the runtime property value name
  ///
  /// # Arguments
  /// * `name` - The runtime property name
  /// * `value` - The runtime property value
  pub fn invoke<N, V>(
    &self,
    handle: &mut NonNull<c_void>,
    name: N,
    value: V,
  ) -> HostFxrResult<()>
  where
    N: IntoFxrBytes<char_t>,
    V: IntoFxrBytes<char_t>,
  {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_set_runtime_property_value` not found");

    let name = name.into_fxr_bytes();
    let name = name.as_ptr();

    let value = value.into_fxr_bytes();
    let value = value.as_ptr();

    let flag = unsafe { symbol(handle.as_mut(), name, value) };

    HostFxrError::from_status(flag)?;

    Ok(())
  }
}

/// Safely wraps `hostfxr_get_runtime_property_value` calls
#[derive(Debug, Clone)]
pub struct GetRuntimePropertyValueSymbol<'lib>(
  Symbol<'lib, dotnet_hostfxr_sys::hostfxr_get_runtime_property_value_fn>,
);

impl<'lib> GetRuntimePropertyValueSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe {
      library.get(b"hostfxr_get_runtime_property_value")?
    }))
  }

  /// Gets the runtime property value by name
  ///
  /// # Arguments
  /// * `name` - The name of the runtime property
  pub fn invoke<N>(&self, handle: &mut NonNull<c_void>, name: N) -> HostFxrResult<String>
  where
    N: IntoFxrBytes<char_t>,
  {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_get_runtime_property_value` not found");

    let name = name.into_fxr_bytes();
    let name = name.as_ptr();
    let mut value: *const char_t = null();

    let flag = unsafe { symbol(handle.as_mut(), name, &mut value as *mut *const _) };

    HostFxrError::from_status(flag)?;

    Ok(value.into_fxr_string())
  }
}

/// Safely wraps `hostfxr_get_runtime_properties` calls
#[derive(Debug, Clone)]
pub struct GetRuntimePropertiesSymbol<'lib>(
  Symbol<'lib, dotnet_hostfxr_sys::hostfxr_get_runtime_properties_fn>,
);

impl<'lib> GetRuntimePropertiesSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe {
      library.get(b"hostfxr_get_runtime_properties")?
    }))
  }

  /// Get all the runtime properties
  pub fn invoke(
    &self,
    handle: &mut NonNull<c_void>,
  ) -> HostFxrResult<HashMap<String, String>> {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_get_runtime_properties` not found");

    let mut properties = HashMap::new();

    let mut count: u64 = 2048;
    let mut keys = vec![null(); count as usize];
    let mut values = vec![null(); count as usize];

    let flag = unsafe {
      symbol(
        handle.as_mut(),
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
        let key = key.into_fxr_string();
        let value = value.into_fxr_string();

        properties.insert(key, value);
      });

    Ok(properties)
  }
}

/// Safely wraps `hostfxr_run_app` calls
#[derive(Debug, Clone)]
pub struct RunAppSymbol<'lib>(Symbol<'lib, dotnet_hostfxr_sys::hostfxr_run_app_fn>);

impl<'lib> RunAppSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe { library.get(b"hostfxr_run_app")? }))
  }

  /// Run app
  pub fn invoke(&self, handle: &mut NonNull<c_void>) -> HostFxrResult<()> {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_run_app` not found");

    HostFxrError::from_status(unsafe { symbol(handle.as_mut()) })
  }
}

/// Safely wraps `hostfxr_get_runtime_delegate` calls
#[derive(Debug, Clone)]
pub struct GetRuntimeDelegateSymbol<'lib>(
  Symbol<'lib, dotnet_hostfxr_sys::hostfxr_get_runtime_delegate_fn>,
);

impl<'lib> GetRuntimeDelegateSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe {
      library.get(b"hostfxr_get_runtime_delegate")?
    }))
  }

  /// Get runtime delegate
  pub fn invoke<D>(&self, handle: &mut NonNull<c_void>) -> HostFxrResult<D::Fn>
  where
    D: RuntimeDelegate,
  {
    let symbol = self
      .0
      .clone()
      .lift_option()
      .expect("Symbol `hostfxr_get_runtime_delegate` not found");

    let mut delegate = MaybeUninit::<D::Fn>::uninit();
    let delegate_ptr = delegate.as_mut_ptr() as *mut _ as *mut *mut _;
    let flag = unsafe { symbol(handle.as_mut(), D::KIND, delegate_ptr) };

    HostFxrError::from_status(flag)?;

    Ok(unsafe { delegate.assume_init() })
  }
}

/// Safely wraps `hostfxr_close` calls
#[derive(Debug, Clone)]
pub struct CloseSymbol<'lib>(Symbol<'lib, dotnet_hostfxr_sys::hostfxr_close_fn>);

impl<'lib> CloseSymbol<'lib> {
  pub fn new(library: &'lib Library) -> HostFxrResult<Self> {
    Ok(Self(unsafe { library.get(b"hostfxr_close")? }))
  }

  /// Close runtime handle
  pub fn invoke(&self, handle: &mut NonNull<c_void>) -> HostFxrResult<()> {
    let symbol = self.0.clone().lift_option();
    match symbol {
      Some(symbol) => HostFxrError::from_status(unsafe { symbol(handle.as_mut()) }),
      None => Ok(()),
    }
  }
}
