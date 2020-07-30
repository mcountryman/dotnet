use std::error::Error;
use std::ptr::null_mut;

/// This API locates the hostfxr library and returns its path
pub fn get_hostfxr_path() -> Result<String, Box<dyn Error>> {
  const MAX_PATH: usize = 260;

  let mut hostfxr_length = MAX_PATH as u64;
  let hostfxr_parameters = null_mut();

  unsafe {
    let mut hostfxr_path_buffer = [0u8; MAX_PATH];
    let result = hostfxr_sys::get_hostfxr_path(
      hostfxr_path_buffer.as_mut_ptr() as *mut i8,
      &mut hostfxr_length,
      hostfxr_parameters,
    );

    if result < 0 {
      bail!("get_hostfxr_path failed with HRESULT {}", result);
    }

    Ok(String::from_utf8(hostfxr_path_buffer.to_vec())?)
  }
}

#[cfg(test)]
mod tests {
  use crate::nethost::get_hostfxr_path;

  #[test]
  fn test_get_hostfxr_path() {
    let path = get_hostfxr_path().expect("Failed to resolve hostfxr path");
    assert!(path.contains("hostfxr"));
  }
}
