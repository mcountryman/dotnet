use libloading::{Library, Symbol};

use crate::error::HostFxrResult;
use crate::parameters::{HostFxrExtraParameters, HostFxrParameters};
use crate::wide::{IntoWideString, WideString};
use crate::{HostFxr, HostFxrLibrary};
use hostfxr_sys::hostfxr_initialize_parameters;
use std::mem::MaybeUninit;
use std::ptr::null;

#[derive(Debug)]
pub struct HostFxrBuilder {
  library: HostFxrLibrary,
}

#[derive(Debug)]
pub struct HostFxrBuilderWithParams<'a> {
  library: HostFxrLibrary,
  extra: Option<HostFxrExtraParameters<'a>>,
  params: HostFxrParameters<'a>,
}

impl HostFxrBuilder {
  pub fn new(library: HostFxrLibrary) -> Self {
    Self { library }
  }

  pub fn command_line<'a, A, I>(self, args: A) -> HostFxrBuilderWithParams<'a>
  where
    A: IntoIterator<Item = I>,
    I: IntoWideString<'a>,
  {
    let iter = args.into_iter().map(IntoWideString::into_wide);
    let iter: Vec<WideString<'a>> = iter.collect();

    HostFxrBuilderWithParams {
      extra: None,
      params: HostFxrParameters::CommandLine(iter),
      library: self.library,
    }
  }

  pub fn runtime_config<'a, R>(self, runtime_config: R) -> HostFxrBuilderWithParams<'a>
  where
    R: IntoWideString<'a>,
  {
    HostFxrBuilderWithParams {
      extra: None,
      params: HostFxrParameters::RuntimeConfig(runtime_config.into_wide()),
      library: self.library,
    }
  }
}

impl<'a> HostFxrBuilderWithParams<'a> {
  pub fn paths<'h: 'a, 'd: 'a, H, D>(mut self, host_path: H, dotnet_root: D) -> Self
  where
    H: IntoWideString<'h>,
    D: IntoWideString<'d>,
  {
    self.extra = Some(HostFxrExtraParameters::new(host_path, dotnet_root));
    self
  }

  pub fn initialize(self) -> HostFxrResult<HostFxr> {
    let params = self.params.clone();
    match params {
      HostFxrParameters::CommandLine(args) => self.init_cli(&args),
      HostFxrParameters::RuntimeConfig(config) => self.init_config(&config),
    }
  }

  fn init_cli(self, args: &[WideString<'a>]) -> HostFxrResult<HostFxr> {
    let initialize_for_dotnet_command_line =
      self.library.initialize_for_dotnet_command_line.clone();
    let initialize_for_dotnet_command_line =
      initialize_for_dotnet_command_line.lift_option().unwrap();

    let mut handle: hostfxr_sys::hostfxr_handle = unsafe { std::mem::zeroed() };
    let argc = args.len() as _;
    let argv: Vec<Vec<_>> = args.into_iter().map(|value| value.into()).collect();
    let mut argv: Vec<_> = argv.into_iter().map(|value| value.as_ptr()).collect();
    let argv = argv.as_mut_ptr();
    let parameters = Self::get_host_parameters(&self.extra);

    let flag = unsafe {
      initialize_for_dotnet_command_line(argc, argv, parameters.as_ptr(), &mut handle)
    };

    if handle.is_null() {
      assert_eq!(flag, 0, "Bad flag and null");
    }

    assert_eq!(flag, 0, "Bad flag");

    Ok(HostFxr::new(handle, self.library))
  }

  fn init_config(self, runtime_config: &WideString<'a>) -> HostFxrResult<HostFxr> {
    let initialize_for_runtime_config =
      self.library.initialize_for_runtime_config.clone();
    let initialize_for_runtime_config =
      initialize_for_runtime_config.lift_option().unwrap();

    let mut handle: hostfxr_sys::hostfxr_handle = unsafe { std::mem::zeroed() };
    let runtime_config: Vec<_> = runtime_config.into();
    let runtime_config = runtime_config.as_ptr();
    let parameters = Self::get_host_parameters(&self.extra);

    let flag = unsafe {
      initialize_for_runtime_config(runtime_config, parameters.as_ptr(), &mut handle)
    };

    if handle.is_null() {
      assert_eq!(flag, 0, "Bad flag and null");
    }

    assert_eq!(flag, 0, "Bad flag");

    Ok(HostFxr::new(handle, self.library))
  }

  fn get_host_parameters(
    extra: &Option<HostFxrExtraParameters<'a>>,
  ) -> MaybeUninit<hostfxr_initialize_parameters> {
    match extra {
      Some(extra) => {
        // TODO: This is probably bad
        let host_path: Vec<_> = extra.host_path.clone().into();
        let host_path: *const _ = host_path.as_ptr();

        let dotnet_root: Vec<_> = extra.dotnet_root.clone().into();
        let dotnet_root: *const _ = dotnet_root.as_ptr();

        MaybeUninit::new(hostfxr_initialize_parameters {
          size: std::mem::size_of::<hostfxr_initialize_parameters>() as _,
          host_path,
          dotnet_root,
        })
      }
      None => MaybeUninit::zeroed(),
    }
  }
}

impl From<HostFxrLibrary> for HostFxrBuilder {
  fn from(library: HostFxrLibrary) -> Self {
    Self { library }
  }
}

#[cfg(test)]
mod tests {
  use crate::{HostFxr, HostFxrLibrary};

  #[test]
  fn test_init_config() {
    std::env::set_current_dir(
      "C:/Users/marvinc/Development/_/dotnet/bridge/bin/Debug/net5.0/",
    )
    .unwrap();

    let hostfxr = HostFxrLibrary::new().unwrap();
    let hostfxr = HostFxr::build(hostfxr)
      .runtime_config(
        "C:/Users/marvinc/Development/_/dotnet/bridge/bin/Debug/net5.0/bridge.runtimeconfig.json",
      )
      .initialize()
      .unwrap();
  }
}
