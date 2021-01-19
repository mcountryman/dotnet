use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotnet_host::ClrObject;
use dotnet_hostfxr::HostFxrLibrary;
use std::{
  fs,
  mem::{size_of, ManuallyDrop},
};

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
      fs::canonicalize("../bridge/bin/Relese/net5.0/bridge.dll")
        .unwrap()
        .to_str()
        .unwrap(),
      "Bridge, bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
      "GetContextHandle",
      "Bridge+GetContextHandleFn, bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null",
    )
    .unwrap();

  let bridge: BridgeContext = get_bridge();

  c.bench_function("add", |i| {
    i.iter(|| {
      let a = ClrObject::from("yuge test");
      let b = ClrObject::from("mawdinawod9h239hjd23j");

      let args = ManuallyDrop::new(vec![a, b]);
      let (argv, argc) = (args.as_ptr(), args.len() as i32);

      match (bridge.add)(argv, argc) {
        ClrObject::Int32(value) => assert_eq!(value, 11),
        obj => panic!("Unexpected obj `{:?}`", obj),
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
