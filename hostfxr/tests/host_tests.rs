use hostfxr::host::HostFxr;
use std::os::raw::c_char;
use std::path::PathBuf;

// namespace HostFxrTest {
//  public class HostFxr {
//    public delegate int TestDelegate(string p1, int p2, long p3);

type TestFn = extern "C" fn(p1: *mut c_char, p2: i32, p3: i64) -> i32;

#[test]
fn test_intialize() {
  let resources = env!("CARGO_MANIFEST_DIR");
  let resources = PathBuf::from(resources).join("tests/resources");
  let runtime_config = resources.join("runtimeconfig.json");
  let host = HostFxr::new("").unwrap();
  let test: &TestFn = host
    .create_delegate(
      resources.join("bin/Debug/netcoreapp3.1/resources.dll"),
      "HostFxrTest.HostFxr",
      "Test",
      "HostFxrTest.TestDelegate",
    )
    .unwrap();
}
