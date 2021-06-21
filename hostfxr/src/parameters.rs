use crate::string::IntoFxrPtr;
use dotnet_hostfxr_sys::hostfxr_initialize_parameters;
use std::{
  borrow::Cow,
  mem::{size_of, MaybeUninit},
};

#[derive(Debug, Clone)]
pub struct HostFxrParameters<'a> {
  pub host_path: Cow<'a, str>,
  pub dotnet_root: Cow<'a, str>,
}

impl<'a> IntoFxrPtr<hostfxr_initialize_parameters> for Option<HostFxrParameters<'a>> {
  unsafe fn into_fxr_ptr(self) -> *const hostfxr_initialize_parameters {
    let parameters = match self {
      Some(parameters) => MaybeUninit::new(hostfxr_initialize_parameters {
        size: size_of::<hostfxr_initialize_parameters>() as u64,
        host_path: parameters.host_path.into_fxr_ptr(),
        dotnet_root: parameters.dotnet_root.into_fxr_ptr(),
      }),
      None => MaybeUninit::zeroed(),
    };

    let ptr = parameters.as_ptr();

    #[allow(clippy::forget_copy)]
    std::mem::forget(parameters);

    ptr
  }
}
