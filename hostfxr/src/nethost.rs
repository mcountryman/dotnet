use std::error::Error;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

/// This API locates the hostfxr library and returns its path
pub fn get_hostfxr_path() -> Result<CString, Box<dyn Error>> {
  const MAX_PATH: usize = 260;

  let mut hostfxr_length = MAX_PATH as u64;
  let hostfxr_parameters = null_mut();

  unsafe {
    let mut hostfxr_path_buffer = [0u8; MAX_PATH + 1];
    let result = hostfxr_sys::get_hostfxr_path(
      hostfxr_path_buffer.as_mut_ptr() as *mut i8,
      &mut hostfxr_length,
      hostfxr_parameters,
    );

    if result < 0 {
      bail!("get_hostfxr_path failed with HRESULT {}", result);
    }

    let hostfxr_path_buffer = &hostfxr_path_buffer[0..hostfxr_length as usize];
    let hostfxr_path = CStr::from_bytes_with_nul(&hostfxr_path_buffer)?;
    let hostfxr_path = CString::from(hostfxr_path);

    Ok(hostfxr_path)
  }
}

#[cfg(test)]
mod tests {
  use crate::nethost::get_hostfxr_path;

  #[test]
  fn test_get_hostfxr_path() {
    let path = get_hostfxr_path().expect("Failed to resolve hostfxr path");
    let path = String::from(path.to_str().unwrap());

    assert!(path.contains("hostfxr"));
  }
}
