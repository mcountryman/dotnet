use crate::Runtime;
use dotnet_hostfxr::{HostFxr, HostFxrLibrary, HostFxrResult};
use once_cell::sync::OnceCell;

static CURRENT: OnceCell<HostFxrRuntime> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct HostFxrRuntime {
  host: HostFxr<'static>,
}

impl Runtime for HostFxrRuntime {
  type Error = dotnet_hostfxr::HostFxrError;

  fn get() -> Result<Self, Self::Error> {
    if let Some(hostfxr) = CURRENT.get() {
      return Ok(hostfxr.clone());
    }

    // Initialize HostFxr
    let host = HostFxrLibrary::get()?;
    let host = host.initialize_runtime_config(
      //
      "bridge/bridge.runtimeconfig.json",
      None,
    )?;

    // Initialize bridge

    let runtime = Self { host };

    Ok(CURRENT.try_insert(runtime).unwrap().clone())
  }

  fn method<M, A>(&self, path: &str) -> Result<M, Self::Error>
  where
    M: crate::method::Method<A>,
    // Not required for this to work but, prevents returning non-plain `fn(..) -> ..` fns by
    // requiring return to be `Fn(..) -> ..`
    M::Fn: crate::method::Method<A>,
  {
    todo!()
  }

  fn release<T>(&self, handle: &crate::gc::GcHandle<T, Self>) -> Result<(), Self::Error> {
    todo!()
  }
}

// pub mod bridge;
// pub use bridge::*;

// pub mod result;
// pub use result::*;

// use dotnet_hostfxr::{HostFxr, HostFxrError, HostFxrLibrary};

// pub trait Host {
//   type Error;

//   fn get_bridge(&self) -> Result<Bridge, Self::Error>;
// }

// pub struct HostFxrHost {
//   host: HostFxr,
// }

// impl HostFxrHost {
//   pub fn new() -> Result<Self, HostFxrError> {
//     Ok(Self {
//       host: HostFxrLibrary::new()?
//         .initialize_runtime_config("bridge/bridge.runtimeconfig.json", None)?,
//     })
//   }
// }

// impl Host for HostFxrHost {
//   type Error = HostFxrError;

//   fn get_bridge(&self) -> Result<Bridge, Self::Error> {
//     type GetBridge = unsafe extern "C" fn() -> Bridge;
//     let get_bridge: GetBridge = self.host.load_assembly_and_get_delegate(
//       "bridge/bin/Release/net5.0/bridge.dll",
//       "Bridge, Bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
//       "GetBridge",
//       "GetBridgeDelegate, Bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
//     )?;

//     Ok(unsafe { get_bridge() })
//   }
// }

// #[cfg(test)]
// mod tests {
//   use super::HostFxrHost;
//   use crate::{Host, ObjectKind};

//   type Add = fn(a: i32, b: i32) -> i32;

//   #[test]
//   fn test_hostfxr_get_bridge() {
//     let host = HostFxrHost::new().expect("Failed to initialize host");
//     let bridge = host.get_bridge().expect("Failed to get bridge");
//     let add = bridge.prepare_invoke::<Add>(
//       "Bridge.Add",
//       ObjectKind::Int32,
//       &[ObjectKind::Int32, ObjectKind::Int32],
//     );

//     assert_eq!(add(2, 2), 4);
//   }
// }
