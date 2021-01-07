use std::env;
use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Result};
use target_lexicon::{Architecture, Environment, OperatingSystem, Triple};
use zip::ZipArchive;

/// Purpose is as follows
/// 1. Find nuget package for build target
///   ie. `x86_64-pc-windows-msvc` -> `runtime.win-x86.Microsoft.NETCore.DotNetAppHost`
/// 2. Download .nupkg
/// 3. Extract contents of `runtime/../native` to `$OUT_DIR/native`.
/// 4. Statically link `nethost`.
/// 5. Dynamically link c++ runtime.
///   TODO: Maybe we should give the user the option for dynamic/static...
fn main() {
  let target = download_runtime("5.0.1").unwrap();
  let target = Path::new(&target);

  // Link nethost static library
  println!("cargo:rustc-link-search=native={}", target.display());
  println!("cargo:rustc-link-lib=static=nethost");

  let out = env::var("OUT_DIR").expect("`$OUT_DIR` not defined.");
  let out = PathBuf::from(out);

  bindgen::builder()
    .header(target.join("nethost.h").to_str().unwrap())
    .derive_copy(true)
    .derive_debug(true)
    .derive_default(true)
    .derive_eq(true)
    .derive_ord(true)
    .derive_partialeq(true)
    .derive_partialord(true)
    .generate_comments(true)
    .whitelist_type("get_hostfxr_parameters")
    .whitelist_function("get_hostfxr_path")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .unwrap()
    .write_to_file(out.join("nethost.rs"))
    .unwrap();

  bindgen::builder()
    .header(target.join("hostfxr.h").to_str().unwrap())
    .derive_copy(true)
    .derive_debug(true)
    .derive_default(true)
    .derive_eq(true)
    .derive_ord(true)
    .derive_partialeq(true)
    .derive_partialord(true)
    .generate_comments(true)
    .dynamic_library_name("hostfxr")
    .whitelist_type("hostfxr_delegate_type")
    .whitelist_type("hostfxr_initialize_parameters")
    .whitelist_type("hostfxr_main_fn")
    .whitelist_type("hostfxr_main_startupinfo_fn")
    .whitelist_type("hostfxr_main_bundle_startupinfo_fn")
    .whitelist_type("hostfxr_error_writer_fn")
    .whitelist_type("hostfxr_set_error_writer_fn")
    .whitelist_type("hostfxr_handle")
    .whitelist_type("hostfxr_initialize_for_dotnet_command_line_fn")
    .whitelist_type("hostfxr_initialize_for_runtime_config_fn")
    .whitelist_type("hostfxr_get_runtime_property_value_fn")
    .whitelist_type("hostfxr_set_runtime_property_value_fn")
    .whitelist_type("hostfxr_get_runtime_properties_fn")
    .whitelist_type("hostfxr_run_app_fn")
    .whitelist_type("hostfxr_get_runtime_delegate_fn")
    .whitelist_type("hostfxr_close_fn")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .unwrap()
    .write_to_file(out.join("hostfxr.rs"))
    .unwrap();

  // Link c++ runtime
  match env::var("CARGO_CFG_TARGET_OS").expect("").as_str() {
    "macos" | "ios" => println!("cargo:rustc-link-lib=dylib=c++"),
    _ => println!("cargo:rustc-link-lib=dylib=vcruntime"),
  };
}

/// Download and extract native folder from runtime nupkg to `$OUT_DIR/native`
fn download_runtime(version: &str) -> Result<String> {
  let target = get_runtime_from_target(&env::var("TARGET")?)?;
  let package_name = format!("runtime.{}.Microsoft.NETCore.DotNetAppHost", target);
  let mut package = fetch_package(&package_name, version)?;

  let out_path = format!("{}/native", &env::var("OUT_DIR")?);
  let native_path = format!("runtimes/{}/native", target);

  std::fs::create_dir_all(&out_path)?;

  for i in 0..package.len() {
    let mut file = package.by_index(i)?;
    if file.name().to_lowercase().starts_with(&native_path) {
      let to = format!("{}/{}", out_path, file.name().replace(&native_path, ""));
      let mut to = File::create(to)?;

      io::copy(&mut file, &mut to)?;
    }
  }

  Ok(out_path.to_owned())
}

/// Download package from name / version.
fn fetch_package(name: &str, version: &str) -> Result<ZipArchive<Cursor<Vec<u8>>>> {
  let url = format!("https://www.nuget.org/api/v2/package/{}/{}", name, version);
  let mut buf = Cursor::new(vec![]);

  match ureq::get(&url).call() {
    Ok(res) => {
      let mut read = res.into_reader();
      io::copy(&mut read, &mut buf)?;
    }
    Err(err) => bail!("Failed to fetch package '{}:{}' {:?}", name, version, err),
  }

  Ok(ZipArchive::new(buf)?)
}

/// Resolve .NET runtime target name from supplied target triple.
///
/// # Notice
/// Tizen is not supported here.  I might add in the future if that's even possible.
fn get_runtime_from_target(target: &str) -> Result<String> {
  let target = Triple::from_str(target).unwrap();

  let arch = match &target.architecture {
    &Architecture::X86_64 => "x64",
    &Architecture::X86_32(_) => "x86",
    &Architecture::Arm(_) => "arm",
    &Architecture::Aarch64(_) => "arm64",
    _ => bail!("Unsupported arch '{}'", &target.architecture),
  };

  let host = match &target.operating_system {
    &OperatingSystem::Linux => match &target.environment {
      &Environment::Musl => "linux-musl",
      _ => "linux",
    },
    &OperatingSystem::Windows => "win",
    &OperatingSystem::MacOSX { .. } => "osx",
    &OperatingSystem::Freebsd => "freebsd",
    _ => bail!("Unsupported os '{}'", &target.operating_system),
  };

  Ok(format!("{}-{}", host, arch))
}
