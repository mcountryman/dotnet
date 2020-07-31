use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
  let root = env!("CARGO_MANIFEST_DIR");
  let root = PathBuf::from(root);
  let target = build_coreclr(&root)?;

  build_bindgen(&target)?;
  build_corerror(&root)?;

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

/// Build coreclr using cmake
fn build_coreclr(root: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
  let arch = get_arch()?;
  let hash = get_commit_hash(root.join("../vendor/dotnet/runtime"))?;

  let native = root.join("../vendor/dotnet/runtime/eng/native");
  let corhost = root.join("../vendor/dotnet/runtime/src/installer/corehost");

  Ok(
    cmake::Config::new(corhost)
      .define("CLR_CMAKE_HOST_ARCH", arch)
      .define("CLR_ENG_NATIVE_DIR", native)
      .define("CLI_CMAKE_HOST_POLICY_VER", "3.0.0")
      .define("CLI_CMAKE_HOST_FXR_VER", "3.0.0")
      .define("CLI_CMAKE_HOST_VER", "3.0.0")
      .define("CLI_CMAKE_COMMON_HOST_VER", "3.0.0")
      .define("CLI_CMAKE_PKG_RID", "3.0.0")
      .define("CLI_CMAKE_COMMIT_HASH", hash)
      .define("CLI_CMAKE_PORTABLE_BUILD", "1")
      .build(),
  )
}

/// Build nethost bindings using bindgen
fn build_bindgen(target: &PathBuf) -> Result<(), Box<dyn Error>> {
  let out = env::var("OUT_DIR")?;
  let out = PathBuf::from(out);
  let bindings = bindgen::builder()
    .header(target.join("corehost/nethost.h").to_str().unwrap())
    .whitelist_type("get_hostfxr_parameters")
    .whitelist_function("get_hostfxr_path")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .unwrap();

  bindings.write_to_file(out.join("bindings.rs"))?;

  Ok(())
}

fn build_corerror(root: &PathBuf) -> Result<(), Box<dyn Error>> {
  let expr = Regex::new(r"#define\s*([^\s]+)\s*([SE])MAKEHR\(([^)]+)\)")?;
  let facility_urt = 0x13;
  let severity_error = 1;
  let severity_success = 0;

  let file =
    File::open(root.join("../vendor/dotnet/runtime/src/coreclr/src/pal/prebuilt/inc/corerror.h"))?;
  let lines = BufReader::new(file).lines();

  let out = env::var("OUT_DIR")?;
  let out = PathBuf::from(out);
  let mut writer = File::create(out.join("hresult.rs"))?;
  //let mut writer = BufWriter::new(writer);

  writeln!(writer, "#[derive(Debug, Copy, Clone)]")?;
  writeln!(writer, "pub enum HRESULT {{")?;

  for line in lines {
    if let Ok(line) = line {
      let matches = expr.captures(line.as_str());
      if matches.is_none() {
        continue;
      }

      let matches = matches.unwrap();
      let name = matches.get(1).unwrap().as_str();
      let kind = matches.get(2).unwrap().as_str();
      let code = matches.get(3).unwrap().as_str();
      let code = code.trim_start_matches("0x");
      let code = i32::from_str_radix(code, 16)?;
      let code = match kind {
        "S" => severity_success << 31 | facility_urt << 16 | code,
        "E" => severity_error << 31 | facility_urt << 16 | code,
        _ => -1,
      };

      // #define CLDB_S_TRUNCATION SMAKEHR(0x1106)
      // #define COR_E_TYPEUNLOADED EMAKEHR(0x1013)
      writeln!(writer, "  {} = {},", name, code)?;
    } else {
      break;
    }
  }

  writeln!(writer, "}}")?;

  Ok(())
}

/// Get target architecture
fn get_arch() -> Result<String, Box<dyn Error>> {
  let arch = env::var("CARGO_CFG_TARGET_ARCH")?;
  match arch.as_str() {
    "x86_64" => Ok("AMD64".to_string()),
    arch => Ok(arch.to_string()),
  }
}

/// Get git commit hash for path
fn get_commit_hash<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
  Ok(
    String::from_utf8(
      Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(path)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?
        .stdout,
    )?
    .trim()
    .to_owned(),
  )
}
