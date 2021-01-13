use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct HostFxrParameters<'a> {
  pub host_path: Cow<'a, str>,
  pub dotnet_root: Cow<'a, str>,
}
