use std::ptr::null_mut;
use std::string::FromUtf16Error;

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
  /// Nethost provided invalid utf16 string indicating that nethost was not built with
  /// unicode support.
  InvalidUtf16(FromUtf16Error),
  /// Hostfxr path exceeds hardcoded path size of `4096`.
  InvalidBufferSize,
}

/// Get `hostfxr` dynamic library path on system using global registration or environment
/// variables.
pub fn get_hostfxr() -> Result<String, GetHostFxrError> {
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

  // Attempt to decode from utf16
  String::from_utf16(&buf[..buf_len as usize - 1]).map_err(GetHostFxrError::InvalidUtf16)
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::get_hostfxr;

  #[test]
  fn test_get_hostfxr() {
    let hostfxr = get_hostfxr().unwrap();
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
