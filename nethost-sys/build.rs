use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
  let out_path = PathBuf::from(env::var("OUT_DIR")?);
  let result = Command::new("dotnet")
    .arg("build")
    .arg(format!("src/nethost.csproj"))
    .spawn()?
    .wait_with_output()?;

  if !result.status.success() {
    panic!("{}", String::from_utf8(result.stderr)?);
  }

  let bindings = bindgen::Builder::default()
    .header(
      out_path
        .join("nethost.h")
        .to_str()
        .expect("Failed to resolve nethost.h"),
    )
    .whitelist_type("get_hostfxr_parameters")
    .whitelist_function("get_hostfxr_path")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Failed to generate bindings");

  bindings.write_to_file(out_path.join("bindings.rs"))?;

  println!("cargo:rustc-link-search=native={}", out_path.display());
  println!("cargo:rustc-link-lib=nethost");

  Ok(())
}
