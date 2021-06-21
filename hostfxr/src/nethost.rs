use crate::{error::HostFxrError, string::IntoFxrString};
use std::ptr::null_mut;

/// MAX_PATH defines buffer size supplied to nethost.  This should be more than enough
/// given that hostfxr will most likely be installed by an installer provided by Microsoft
/// into a common binary directory.
///
/// Good places to test are in environments with NIX or xbps package managers.
const MAX_PATH: usize = 4096;

/// Get `hostfxr` dynamic library path on system using global registration or environment
/// variables.
pub fn get_hostfxr_path() -> Result<String, HostFxrError> {
  let mut buf = vec![0; MAX_PATH];
  let mut buf_len = buf.len() as u64;
  let status = unsafe {
    dotnet_hostfxr_sys::get_hostfxr_path(
      buf.as_mut_ptr(),
      &mut buf_len as *mut _,
      null_mut(),
    )
  };

  HostFxrError::from_status(status)?;

  Ok(buf[..buf_len as usize - 1].into_fxr_string())
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::get_hostfxr_path;

  #[test]
  fn test_get_hostfxr() {
    let hostfxr = get_hostfxr_path().unwrap();
    let hostfxr = Path::new(&hostfxr);
    let hostfxr = hostfxr
      .file_name()
      .expect("Expected filename")
      .to_string_lossy();

    match std::env::consts::OS {
      "macos" => assert_eq!(hostfxr, "libhostfxr.dylib"),
      "windows" => assert_eq!(hostfxr, "hostfxr.dll"),
      _ => assert_eq!(hostfxr, "libhostfxr.so"),
    };
  }
}
