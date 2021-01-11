use crate::string::IntoStr;
use std::ffi::NulError;
use std::ptr::null_mut;

/// MAX_PATH defines buffer size supplied to nethost.  This should be more than enough
/// given that hostfxr will most likely be installed by an installer provided by Microsoft
/// into a common binary directory.
///
/// Good places to test are in environments with NIX or xbps package managers.
const MAX_PATH: usize = 4096;

#[derive(Debug)]
pub enum GetHostFxrError {
  /// Nethost returned non-zero status code.
  Unexpected(i32),
  /// Hostfxr path exceeds hardcoded path size of `4096`.
  InvalidBufferSize,
  /// Nethost provided invalid string.
  InvalidPath(NulError),
}

/// Get `hostfxr` dynamic library path on system using global registration or environment
/// variables.
pub fn get_hostfxr_path() -> Result<String, GetHostFxrError> {
  let mut buf = vec![0; MAX_PATH];
  let mut buf_len = buf.len() as u64;
  let code = unsafe {
    hostfxr_sys::get_hostfxr_path(buf.as_mut_ptr(), &mut buf_len as *mut _, null_mut())
  };

  // Check status code for `HostApiBufferTooSmall`
  #[allow(overflowing_literals)]
  if code == 0x80008098 as i32 {
    return Err(GetHostFxrError::InvalidBufferSize);
  }

  // Check status code for non-success
  if code != 0 {
    return Err(GetHostFxrError::Unexpected(code));
  }

  Ok(buf[..buf_len as usize - 1].into_str().to_string())
}

impl From<()> for GetHostFxrError {
  fn from(_: ()) -> Self {
    Self::Unexpected(-1)
  }
}

impl From<NulError> for GetHostFxrError {
  fn from(inner: NulError) -> Self {
    Self::InvalidPath(inner)
  }
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
      "macos" => assert_eq!(hostfxr, "hostfxr.dylib"),
      "windows" => assert_eq!(hostfxr, "hostfxr.dll"),
      _ => assert_eq!(hostfxr, "hostfxr.so"),
    };
  }
}
