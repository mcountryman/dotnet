use crate::wide::{IntoWideString, WideString};

#[derive(Debug, Clone)]
pub enum HostFxrParameters<'a> {
  CommandLine(Vec<WideString<'a>>),
  RuntimeConfig(WideString<'a>),
}

#[derive(Debug, Clone)]
pub struct HostFxrExtraParameters<'a> {
  pub host_path: WideString<'a>,
  pub dotnet_root: WideString<'a>,
}

impl<'a> HostFxrExtraParameters<'a> {
  pub fn new<'h: 'a, 'd: 'a, H, D>(host_path: H, dotnet_root: D) -> Self
  where
    H: IntoWideString<'h>,
    D: IntoWideString<'d>,
  {
    Self {
      host_path: host_path.into_wide(),
      dotnet_root: dotnet_root.into_wide(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct HostFxrParametersBuilder<'a> {
  pub(crate) extra: Option<HostFxrExtraParameters<'a>>,
  pub(crate) parameters: Option<HostFxrParameters<'a>>,
}
