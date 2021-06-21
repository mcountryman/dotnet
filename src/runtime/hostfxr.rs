use super::bridge::Bridge;
use crate::{
  method::Method,
  runtime::bridge::{self, BridgeError},
  Runtime,
};
use dotnet_hostfxr::{HostFxr, HostFxrLibrary};
use once_cell::sync::OnceCell;
use std::{ffi::c_void, ptr::NonNull, sync::Arc};

static CURRENT: OnceCell<HostFxrRuntime> = OnceCell::new();

#[derive(thiserror::Error, Debug)]
pub enum HostFxrError {
  #[error(transparent)]
  Bridge(#[from] BridgeError),
  #[error("Bridge initialization got null pointer")]
  BridgeNone,
  #[error(transparent)]
  HostFxr(#[from] dotnet_hostfxr::HostFxrError),
}

#[derive(Clone)]
pub struct HostFxrRuntime<'rt> {
  host: Arc<HostFxr<'rt>>,
  bridge: Bridge<'rt, HostFxrRuntime<'rt>>,
}

impl<'rt> Runtime for HostFxrRuntime<'rt> {
  type Error = HostFxrError;

  fn get() -> Result<Self, Self::Error> {
    if let Some(hostfxr) = CURRENT.get() {
      return Ok(hostfxr.clone());
    }

    // Initialize HostFxr
    let host = HostFxrLibrary::get()?;
    let host = Arc::new(host.initialize_runtime_config(
      //
      "bridge/bridge.runtimeconfig.json",
      None,
    )?);

    let bridge = get_bridge(&host)?;
    let bridge = match bridge {
      Some(bridge) => bridge,
      None => return Err(HostFxrError::BridgeNone),
    };

    // Initialize bridge
    let runtime = HostFxrRuntime { host, bridge };
    match CURRENT.try_insert(runtime) {
      Ok(runtime) => Ok(runtime.clone()),
      Err((runtime, _)) => Ok(runtime.clone()),
    }
  }

  fn method<M, A>(&self, path: &str) -> Result<&M, Self::Error>
  where
    M: Method<A>,
    // Not required for this to work but, prevents returning non-plain `fn(..) -> ..` fns by
    // requiring return to be `Fn(..) -> ..`
    M::Fn: Method<A>,
  {
    let mut types = M::arg_type_ids().to_vec();
    types.push(M::ret_type_id());

    Ok(self.bridge.get_method(path, types)?)
  }

  fn release<T>(&self, handle: &mut NonNull<T>) -> Result<(), Self::Error> {
    Ok(self.bridge.release(handle)?)
  }
}

fn get_bridge<'rt, H: AsRef<HostFxr<'rt>>, R: Runtime>(
  host: H,
) -> Result<Option<Bridge<'rt, R>>, HostFxrError> {
  type GetBridge = unsafe extern "C" fn() -> *mut c_void;

  let host = host.as_ref();
  let bridge: GetBridge = host.load_assembly_and_get_delegate(
    bridge::get_bridge_assembly_path(),
    bridge::get_bridge_type_name(),
    bridge::get_bridge_method_name(),
    bridge::get_bridge_delegate_name(),
  )?;

  unsafe {
    let bridge = bridge();
    let bridge = Bridge::from_handle(bridge);

    Ok(bridge)
  }
}

#[cfg(test)]
mod tests {
  use std::ptr::NonNull;

  use super::HostFxrRuntime;
  use crate::Runtime;

  #[test]
  fn test_get() {
    let rt = HostFxrRuntime::get().unwrap();
    let mut test = 0;
    let mut test = NonNull::new(&mut test).unwrap();

    rt.release(&mut test).unwrap();
  }
}
