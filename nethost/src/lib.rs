use std::error::Error;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;

const MAX_PATH: u64 = 260u64;

/// Locates hostfxr library
pub fn get_hostfxr_path() -> Result<String, Box<dyn Error>> {
  let buffer = Vec::<i8>::with_capacity(MAX_PATH as usize);
  let mut length = MAX_PATH;
  let mut buffer = ManuallyDrop::new(buffer);

  unsafe {
    let result = nethost_sys::get_hostfxr_path(buffer.as_mut_ptr(), &mut length, null_mut());

    if result < 0 {
      panic!("get_hostfxr_path failed with code {}", result);
    }

    let path = String::from_raw_parts(
      buffer.as_mut_ptr() as *mut u8,
      length as usize - 1,
      buffer.capacity(),
    );

    std::mem::drop(buffer);

    Ok(path)
  }
}

#[cfg(test)]
mod tests {
  use crate::get_hostfxr_path;
  use std::error::Error;
  use std::path::PathBuf;

  #[test]
  fn test_get_hostfxr_path() -> Result<(), Box<dyn Error>> {
    let path = get_hostfxr_path()?;
    let path = PathBuf::from(path);
    let file_name = path.file_stem().unwrap().to_str().unwrap();

    assert!(file_name.ends_with("hostfxr"));

    Ok(())
  }
}
