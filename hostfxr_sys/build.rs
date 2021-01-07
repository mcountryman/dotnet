use std::env;
use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
  let target = install_package("5.0.1");
  let target = Path::new(&target);

  let out = env::var("OUT_DIR").expect("`$OUT_DIR` not defined.");
  let out = PathBuf::from(out);

  bindgen::builder()
    .header(target.join("nethost.h").to_str().unwrap())
    .header(target.join("hostfxr.h").to_str().unwrap())
    // Derives
    .derive_copy(true)
    .derive_debug(true)
    .derive_default(true)
    .derive_eq(true)
    .derive_ord(true)
    .derive_partialeq(true)
    .derive_partialord(true)
    // Comments
    .generate_comments(true)
    // Whitelist nethost.h items
    .whitelist_type("get_hostfxr_parameters")
    .whitelist_function("get_hostfxr_path")
    // Whitelist hostfxr.h items
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
    .expect("Failed to generate bindings")
    .write_to_file(out.join("bindings.rs"))
    .expect("Failed to write bindings");

  // Link nethost static library
  println!("cargo:rustc-link-search=native={}", target.display());
  link("nethost");

  // Link c++ runtime
  match &env::var("CARGO_CFG_TARGET_OS").expect("`$CARGO_CFG_TARGET_OS` undefined")[..] {
    "ios" => link("c++"),
    "macos" => link("c++"),
    _ => link("vcruntime"),
  };
}

fn link(name: &str) {
  if get_link_static(name) {
    println!("cargo:rustc-link-lib=static={}", name)
  } else {
    println!("cargo:rustc-link-lib=dylib={}", name)
  }
}

/// Determines if library should be linked statically.
///
/// 1. Check env var `$name_STATIC`
/// 2. Check feature flag `static`
/// 3. Check if non-musl linux
fn get_link_static(name: &str) -> bool {
  // Environment item overrides all
  if env::var(format!("{}_STATIC", name)).is_ok() {
    return true;
  }

  if cfg!(feature = "static") {
    return true;
  }

  let target = env::var("TARGET").expect("`$TARGET` undefined");
  let target = Triple::from_str(&target).unwrap();

  match target.operating_system {
    OperatingSystem::Linux => matches!(target.environment, Environment::Musl),
    OperatingSystem::Windows => false,
    _ => true,
  }
}

/// Download and extract native folder from runtime nupkg to `$OUT_DIR/native`
fn install_package(version: &str) -> String {
  let target = env::var("TARGET").expect("`$TARGET` undefined");
  let out_dir = env::var("OUT_DIR").expect("`$OUT_DIR` undefined");

  let target = get_runtime_from_target(&target);
  let package_name = format!("runtime.{}.Microsoft.NETCore.DotNetAppHost", target);
  let mut package = fetch_package(&package_name, version);

  let out_path = format!("{}/native", &out_dir);
  let native_path = format!("runtimes/{}/native", target);

  std::fs::create_dir_all(&out_path)
    .unwrap_or_else(|_| panic!("Failed to create `{}` dir", &out_path));

  for i in 0..package.len() {
    let mut file = package.by_index(i).unwrap();
    if file.name().to_lowercase().starts_with(&native_path) {
      let to = format!("{}/{}", out_path, file.name().replace(&native_path, ""));
      let mut to = File::create(to)
        .unwrap_or_else(|_| panic!("Failed to extract `{}`", file.name()));

      io::copy(&mut file, &mut to)
        .unwrap_or_else(|_| panic!("Failed to extract `{}`", file.name()));
    }
  }

  out_path
}

/// Download package from name / version.
fn fetch_package(name: &str, version: &str) -> ZipArchive<Cursor<Vec<u8>>> {
  let url = format!("https://www.nuget.org/api/v2/package/{}/{}", name, version);
  let mut buf = Cursor::new(vec![]);

  match ureq::get(&url).call() {
    Ok(res) => {
      let mut read = res.into_reader();
      io::copy(&mut read, &mut buf).expect("Failed to read response");
    }
    Err(err) => panic!("Failed to fetch package '{}:{}' {:?}", name, version, err),
  }

  ZipArchive::new(buf).expect("Nuget package not a zip file")
}

/// Resolve .NET runtime target name from supplied target triple.
///
/// # Notice
/// Tizen is not supported here.  I might add in the future if that's even possible.
fn get_runtime_from_target(target: &str) -> String {
  let target = Triple::from_str(target).unwrap();

  let arch = match target.architecture {
    Architecture::X86_64 => "x64",
    Architecture::X86_32(_) => "x86",
    Architecture::Arm(_) => "arm",
    Architecture::Aarch64(_) => "arm64",
    _ => panic!("Unsupported arch '{}'", &target.architecture),
  };

  let host = match target.operating_system {
    OperatingSystem::Linux => match target.environment {
      Environment::Musl => "linux-musl",
      _ => "linux",
    },
    OperatingSystem::Windows => "win",
    OperatingSystem::MacOSX { .. } => "osx",
    OperatingSystem::Freebsd => "freebsd",
    _ => panic!("Unsupported os '{}'", &target.operating_system),
  };

  format!("{}-{}", host, arch)
}
