use cmake::Config;
use std::process::{Command, Stdio};
use std::fs;
use std::io::Read;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
  let hash = String::from_utf8(
    Command::new("git")
      .arg("rev-parse")
      .arg("HEAD")
      .current_dir("../vendor/dotnet/runtime")
      .stdout(Stdio::piped())
      .spawn()?
      .wait_with_output()?
      .stdout
  )?;

  let hash = hash.trim();
  let corhost = get_absolute("../vendor/dotnet/runtime/src/installer/corehost")?;
  let native = get_absolute("../vendor/dotnet/runtime/eng/native")?;

  let dest = Config::new(corhost)
    .define("CLR_CMAKE_HOST_ARCH", "x64")
    .define("CLR_ENG_NATIVE_DIR", native)
    .define("CLI_CMAKE_HOST_POLICY_VER", "3.0.0")
    .define("CLI_CMAKE_HOST_FXR_VER", "3.0.0")
    .define("CLI_CMAKE_HOST_VER", "3.0.0")
    .define("CLI_CMAKE_COMMON_HOST_VER", "3.0.0")
    .define("CLI_CMAKE_PKG_RID", "3.0.0")
    .define("CLI_CMAKE_COMMIT_HASH", hash)
    .define("CLI_CMAKE_PORTABLE_BUILD", "1")
    .build();

  Ok(())
}

fn get_absolute<S: Into<String>>(path: S) -> Result<String, Box<dyn Error>> {
  let path = PathBuf::from(path.into());
  let path = fs::canonicalize(&path).expect("the path couldn't be canonicalized");
  let path = path.to_str().expect("the path couldn't be converted to &str");

  Ok(path.replace("\\\\?\\", ""))
}
