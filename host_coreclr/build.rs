use std::path::{Path, PathBuf};
use std::error::Error;
use std::process::Command;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
  let result = Command::new("dotnet")
    .arg("build")
    .arg(format!("tools/nethost.csproj"))
    .spawn()?
    .wait_with_output()?;

  if !result.status.success() {
    panic!("{}", String::from_utf8(result.stderr)?);
  }

  let bindings = bindgen::Builder::default()
    .header("lib/nethost.h")
    .whitelist_type("get_hostfxr_parameters")
    .whitelist_function("get_hostfxr_path")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Failed to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR")?);

  bindings
    .write_to_file(out_path.join("bindings.rs"))?;

  Ok(())
}
