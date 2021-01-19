use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum ClrObject {
  Char(u8),
  Byte(u8),
  SByte(i8),
  Boolean(bool),
  Short(i16),
  UShort(u16),
  Int32(i32),
  UInt32(u32),
  Int64(i64),
  UInt64(u64),
  Float(f32),
  Double(f64),
  Decimal(f64),
  String { value: *const u8, length: u64 },
}

impl<S: AsRef<str>> From<S> for ClrObject {
  fn from(value: S) -> Self {
    let value = ManuallyDrop::new(value.as_ref().to_owned());
    let value_ptr = value.as_ptr();
    let value_len = value.len() as _;

    Self::String {
      value: value_ptr,
      length: value_len,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ClrObject;
  use dotnet_hostfxr::HostFxrLibrary;
  use std::{
    fs,
    mem::{size_of, ManuallyDrop},
    process::{Command, Stdio},
  };

  type Add = extern "C" fn(argv: *const ClrObject, argc: i32) -> ClrObject;
  type GetBridge = extern "C" fn() -> BridgeContext;

  #[repr(C)]
  struct BridgeContext {
    pub add: Add,
  }

  #[test]
  fn test_prelim() {
    let hostfxr = HostFxrLibrary::new().unwrap();
    let hostfxr = hostfxr
      .initialize_runtime_config(
        fs::canonicalize("../bridge/bridge.runtimeconfig.json")
          .unwrap()
          .to_str()
          .unwrap(),
        None,
      )
      .unwrap();

    let get_bridge: GetBridge = hostfxr
    .load_assembly_and_get_delegate(
      fs::canonicalize("../bridge/bin/Debug/net5.0/bridge.dll")
        .unwrap()
        .to_str()
        .unwrap(),
      "Bridge, bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
      "GetContextHandle",
      "Bridge+GetContextHandleFn, bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
    )
    .unwrap();

    let bridge: BridgeContext = get_bridge();

    let a = ClrObject::Int32(i32::MAX);
    let b = ClrObject::Int32(7);
    let c = ClrObject::Char(69 as _);
    let d: ClrObject = "test tickles".into();

    let args = ManuallyDrop::new(vec![c]);
    let (argv, argc) = (args.as_ptr(), args.len());

    let buf = unsafe {
      std::slice::from_raw_parts(
        argv as *const u8,
        argc * std::mem::size_of::<ClrObject>(),
      )
    };

    println!("buf: {:?}", buf);

    match (bridge.add)(argv, argc as _) {
      ClrObject::Int32(value) => assert_eq!(value, 10),
      obj => panic!("Unexpected obj `{:?}`", obj),
    }
  }
}
