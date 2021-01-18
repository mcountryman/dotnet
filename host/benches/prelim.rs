use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotnet_host::ClrObject;
use dotnet_hostfxr::HostFxrLibrary;
use std::{fs, mem::{ManuallyDrop, size_of}};

type Add = extern "C" fn(argv: *const ClrObject, argc: i32) -> ClrObject;
type GetBridge = extern "C" fn() -> BridgeContext;

#[repr(C)]
struct BridgeContext {
  pub add: Add,
}

fn criterion_benchmark(c: &mut Criterion) {
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

  // c.bench_function("add_std", |b| {
  //   b.iter(|| {
  //     let a = CString::new("hello ").unwrap();
  //     let a = a.into_raw();

  //     let b = CString::new("world").unwrap();
  //     let b = b.into_raw();

  //     black_box((bridge.add_std)(a, b));
  //   })
  // });

  c.bench_function("add", |i| {
    i.iter(|| {
      let a = ClrObject::Int32(4);
      let b = ClrObject::Int32(7);

      let args = ManuallyDrop::new(vec![a, b]);
      let (argv, argc) = (args.as_ptr(), args.len() as i32);

      let arg_d = argv as *const u8;
      let arg_d = unsafe {
        let len = size_of::<ClrObject>() / 8;
        std::slice::from_raw_parts(arg_d, len);
      };

      panic!("size: {:?}\nargv: {:?}", size_of::<ClrObject>(), arg_d);

      match (bridge.add)(argv, argc) {
        ClrObject::Int32(value) => assert_eq!(value, 10),
        obj => panic!("Unexpected obj `{:?}`", obj),
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
