use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
  // Get commit hash from git
  let hash = String::from_utf8(
    Command::new("git")
      .arg("rev-parse")
      .arg("HEAD")
      .current_dir("../vendor/dotnet/runtime")
      .stdout(Stdio::piped())
      .spawn()?
      .wait_with_output()?
      .stdout,
  )?;

  let out = env::var("OUT_DIR")?;
  let out = PathBuf::from(out);
  let hash = hash.trim();
  let arch = get_arch()?;
  let native = get_absolute("../vendor/dotnet/runtime/eng/native")?;
  let corhost = get_absolute("../vendor/dotnet/runtime/src/installer/corehost")?;

  // Build dotnet hostfxr CMakeList.txt
  let target = cmake::Config::new(corhost)
    .define("CLR_CMAKE_HOST_ARCH", arch)
    .define("CLR_ENG_NATIVE_DIR", native)
    .define("CLI_CMAKE_HOST_POLICY_VER", "3.0.0")
    .define("CLI_CMAKE_HOST_FXR_VER", "3.0.0")
    .define("CLI_CMAKE_HOST_VER", "3.0.0")
    .define("CLI_CMAKE_COMMON_HOST_VER", "3.0.0")
    .define("CLI_CMAKE_PKG_RID", "3.0.0")
    .define("CLI_CMAKE_COMMIT_HASH", hash)
    .define("CLI_CMAKE_PORTABLE_BUILD", "1")
    .build();

  // Generate bindings
  let bindings = bindgen::builder()
    .header(target.join("corehost/nethost.h").to_str().unwrap())
    .whitelist_type("get_hostfxr_parameters")
    .whitelist_function("get_hostfxr_path")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .unwrap();

  bindings.write_to_file(out.join("bindings.rs"))?;

  // Link nethost static library
  println!(
    "cargo:rustc-link-search=native={}",
    target.join("corehost").display()
  );
  println!("cargo:rustc-link-lib=static=nethost");

  // Link c++
  match env::var("CARGO_CFG_TARGET_OS")?.as_str() {
    "macos" | "ios" => println!("cargo:rustc-link-lib=dylib=c++"),
    "linux" => println!("cargo:rustc-link-lib=dylib=stdc++"),
    _ => unimplemented!(),
  };

  Ok(())
}

fn get_arch() -> Result<String, Box<dyn Error>> {
  let arch = env::var("CARGO_CFG_TARGET_ARCH")?;
  match arch.as_str() {
    "x86_64" => Ok("AMD64".to_string()),
    arch => Ok(arch.to_string()),
  }
}

fn get_absolute<S: Into<String>>(path: S) -> Result<String, Box<dyn Error>> {
  let path = PathBuf::from(path.into());
  let path = fs::canonicalize(&path).expect(&*format!(
    "the path '{}' couldn't be canonicalized",
    path.display()
  ));
  let path = path.to_str().expect(&*format!(
    "the path '{}' couldn't be converted to &str",
    path.display()
  ));

  Ok(path.replace("\\\\?\\", ""))
}
