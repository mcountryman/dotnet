use std::ptr::null_mut;
use std::str::Utf8Error;
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
  let code =
    unsafe { get_hostfxr_path(buf.as_mut_ptr(), &mut buf_len as *mut _, null_mut()) };

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
  String::from_utf16(&buf[..buf_len as usize])
    .map_err(|err| GetHostFxrError::InvalidUtf16(err))
}

extern "C" {
  fn get_hostfxr_path(
    buffer: *mut u16,
    buffer_size: *mut u64,
    parameters: *const u32,
  ) -> i32;
}

#[cfg(test)]
mod tests {
  use crate::get_hostfxr;

  #[test]
  fn test_get_hostfxr() {
    let hostfxr = get_hostfxr().unwrap();
    match std::env::consts::OS {
      "macos" => assert!(hostfxr.ends_with("hostfxr.dylib")),
      "windows" => assert!(hostfxr.ends_with("hostfxr.dll")),
      _ => assert!(hostfxr.ends_with("hostfxr.so")),
    };
  }
}
