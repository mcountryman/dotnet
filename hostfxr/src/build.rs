use libloading::{Library, Symbol};

use crate::error::HostFxrResult;
use crate::parameters::{HostFxrExtraParameters, HostFxrParameters};
use crate::wide::{IntoWideString, WideString};
use crate::{HostFxr, HostFxrLibrary};

pub struct HostFxrBuilder<'a> {
  library: HostFxrLibrary<'a>,
}

pub struct HostFxrBuilderWithParams<'a, 'b> {
  library: HostFxrLibrary<'a>,
  extra: Option<HostFxrExtraParameters<'b>>,
  params: HostFxrParameters<'b>,
}

impl<'a> HostFxrBuilder<'a> {
  pub fn command_line<'r, A, I>(self, args: A) -> HostFxrBuilderWithParams<'a, 'r>
  where
    A: IntoIterator<Item = I>,
    I: IntoWideString<'r>,
  {
    let iter = args.into_iter().map(IntoWideString::into_wide);
    let iter: Vec<WideString<'r>> = iter.collect();

    HostFxrBuilderWithParams {
      extra: None,
      params: HostFxrParameters::CommandLine(iter),
      library: self.library,
    }
  }

  pub fn runtime_config<'r, R>(
    self,
    runtime_config: R,
  ) -> HostFxrBuilderWithParams<'a, 'r>
  where
    R: IntoWideString<'r>,
  {
    HostFxrBuilderWithParams {
      extra: None,
      params: HostFxrParameters::RuntimeConfig(runtime_config.into_wide()),
      library: self.library,
    }
  }
}

impl<'a, 'b> HostFxrBuilderWithParams<'a, 'b> {
  pub fn paths<'h: 'b, 'd: 'b, H, D>(&mut self, host_path: H, dotnet_root: D) -> &mut Self
  where
    H: IntoWideString<'h>,
    D: IntoWideString<'d>,
  {
    self.extra = Some(HostFxrExtraParameters::new(host_path, dotnet_root));
    self
  }

  pub fn initialize(self) -> HostFxrResult<HostFxr<'a>> {
    let extra = self.extra;
    match self.params {
      HostFxrParameters::CommandLine(args) => Self::init_command_line(args, extra),
      HostFxrParameters::RuntimeConfig(runtime_config) => {
        Self::init_runtime_config(runtime_config, extra)
      }
    }
  }

  fn init_command_line(
    args: Vec<WideString<'b>>,
    extra: Option<HostFxrExtraParameters<'b>>,
  ) -> HostFxrResult<HostFxr<'a>> {
    unimplemented!()
  }

  fn init_runtime_config(
    runtime_config: WideString<'b>,
    extra: Option<HostFxrExtraParameters<'b>>,
  ) -> HostFxrResult<HostFxr<'a>> {
    unimplemented!()
  }
}
