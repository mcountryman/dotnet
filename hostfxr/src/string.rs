//! Compile time `char_t` conversion to and from std types.
//!
//! Due to `hostfxr` APIs using `c_wchar` and `c_char` in different builds an abstraction
//! for both are needed.  
use crate::parameters::HostFxrParameters;
use dotnet_hostfxr_sys::hostfxr_initialize_parameters;
use std::mem::size_of;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::slice::from_raw_parts;

/// A value-to-vec conversion with items of variable size.
pub trait IntoBytes<T> {
  /// Performs the conversion.
  fn into_bytes(self) -> Vec<T>;
}

/// A value-to-pointer conversion with items of variable size.
pub trait IntoPtr<T> {
  /// Performs the conversion.
  unsafe fn into_ptr(self) -> *const T;
}

/// Convert to string, consuming value.
pub trait IntoString<'a> {
  /// Performs the conversion.
  fn into_string(self) -> String;
}

impl<'a> IntoString<'a> for &'a [c_char] {
  /// Copies nul terminated `c_char` ptr into utf8 decoded string.
  fn into_string(self) -> String {
    let len = self.len();
    let buf = unsafe { from_raw_parts(self.as_ptr() as *const _, len) };

    String::from_utf8_lossy(buf).to_string()
  }
}

impl<'a> IntoString<'a> for *const c_char {
  /// Copies nul terminated `c_char` ptr into utf8 decoded string.
  ///
  /// # Safety
  /// If supplied ptr is not nul terminated, nul terminator loop will continue forever or
  /// until segfault.
  fn into_string(self) -> String {
    let len = (0..).position(|i| unsafe { *self.offset(i) == 0 }).unwrap();
    let buf = unsafe {
      let mut v = Vec::with_capacity(len);
      std::ptr::copy(self as *const _, v.as_mut_ptr(), len);

      v.set_len(len);
      v
    };

    String::from_utf8_lossy(&buf).to_string()
  }
}

#[cfg(windows)]
impl<'a> IntoString<'a> for *const u16 {
  /// Copies nul terminated `c_wchar` ptr into wtf8 decoded string.
  ///
  /// # Safety
  /// If supplied ptr is not nul terminated, nul terminator loop will continue forever or
  /// until segfault.
  fn into_string(self) -> String {
    let len = (0..).position(|i| unsafe { *self.offset(i) == 0 }).unwrap();
    let buf = unsafe {
      let mut v = Vec::with_capacity(len);

      std::ptr::copy(self, v.as_mut_ptr(), len);

      v.set_len(len);
      v
    };

    buf.into_string()
  }
}

#[cfg(windows)]
impl<'a> IntoString<'a> for &'a [u16] {
  fn into_string(self) -> String {
    use std::os::windows::prelude::*;

    let buf = std::ffi::OsString::from_wide(self);
    let buf = buf.to_string_lossy();
    let buf = buf.to_string();

    buf
  }
}

#[cfg(windows)]
impl<'a> IntoString<'a> for Vec<u16> {
  fn into_string(self) -> String {
    self.as_slice().into_string()
  }
}

impl<R: AsRef<str>> IntoBytes<c_char> for R {
  fn into_bytes(self) -> Vec<c_char> {
    self
      .as_ref()
      .as_bytes()
      .iter()
      .map(|ch| *ch as c_char)
      .chain(Some(0))
      .collect()
  }
}

#[cfg(windows)]
impl<R: AsRef<str>> IntoBytes<u16> for R {
  fn into_bytes(self) -> Vec<u16> {
    use std::os::windows::prelude::*;

    let buf = self.as_ref();
    let buf = std::ffi::OsString::from(buf);
    let buf: Vec<_> = buf.encode_wide().chain(Some(0)).collect();

    buf
  }
}

impl<R: IntoBytes<c_char>> IntoPtr<c_char> for R {
  unsafe fn into_ptr(self) -> *const c_char {
    let buf = self.into_bytes();
    let ptr = buf.as_ptr();

    std::mem::forget(buf);

    ptr
  }
}

#[cfg(windows)]
impl<R: IntoBytes<u16>> IntoPtr<u16> for R {
  unsafe fn into_ptr(self) -> *const u16 {
    let buf = self.into_bytes();
    let ptr = buf.as_ptr();

    std::mem::forget(buf);

    ptr
  }
}

impl<'a> IntoPtr<hostfxr_initialize_parameters> for Option<HostFxrParameters<'a>> {
  unsafe fn into_ptr(self) -> *const hostfxr_initialize_parameters {
    let parameters = match self {
      Some(parameters) => MaybeUninit::new(hostfxr_initialize_parameters {
        size: size_of::<hostfxr_initialize_parameters>() as u64,
        host_path: parameters.host_path.into_ptr(),
        dotnet_root: parameters.dotnet_root.into_ptr(),
      }),
      None => MaybeUninit::zeroed(),
    };

    let ptr = parameters.as_ptr();

    #[allow(clippy::forget_copy)]
    std::mem::forget(parameters);

    ptr
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::slice;

  #[test]
  fn test_into_str_from_c_char() {
    let cases = vec![
      (&b"Hello World"[..], "Hello World"),
      (&b"Hello World\0"[..], "Hello World\0"),
      (&b"Hello \0World"[..], "Hello \0World"),
      (&b"\0"[..], "\0"),
      (&b""[..], ""),
    ];

    for (buf, expected) in cases {
      let buf = unsafe { slice::from_raw_parts(buf.as_ptr() as *const i8, buf.len()) };

      let actual = buf.into_string();
      assert_eq!(actual, expected);
    }
  }

  #[test]
  #[cfg(windows)]
  fn test_into_str_from_wide() {
    let cases: Vec<(&[u16], &str)> = vec![
      (wchar::wch!("Hello World"), "Hello World"),
      (wchar::wch!("Hello World\0"), "Hello World\u{0}"),
      (wchar::wch!("Hello \0World"), "Hello \u{0}World"),
      (wchar::wch!("\0"), "\u{0}"),
      (wchar::wch!(""), ""),
    ];

    for (buf, expected) in cases {
      let actual = buf.into_string();
      assert_eq!(actual, expected);
    }
  }

  #[test]
  fn test_into_bytes_from_c_char() {
    let cases = vec!["Hello World", "Hello World\0", "Hello \0World", "\0", ""];

    for case in cases {
      let buf: Vec<c_char> = case.into_bytes();
      let actual = buf.into_iter().position(|ch| ch == 0).unwrap();
      let expected = case
        .chars()
        .position(|ch| ch == '\0')
        .unwrap_or_else(|| case.len());

      assert_eq!(
        expected, actual,
        "Null terminated lengths don't match for `{}`",
        case
      );
    }
  }

  #[test]
  #[cfg(windows)]
  fn test_into_bytes_from_wide() {
    let cases = vec!["Hello World", "Hello World\0", "Hello \0World", "\0", ""];

    for case in cases {
      let buf: Vec<u16> = case.into_bytes();
      let actual = buf.iter().position(|ch| *ch == 0).unwrap();
      let expected = case
        .chars()
        .position(|ch| ch == '\0')
        .unwrap_or_else(|| case.len());

      assert_eq!(
        expected, actual,
        "Null terminated lengths don't match for `{}`",
        case
      );

      assert_eq!(&buf[buf.len() - 1], &0, "Bytes are not null terminated");
    }
  }

  #[test]
  fn test_into_str_from_c_char_ptr() {
    let cases = vec![
      (&b"Hello World\0"[..], "Hello World"),
      (&b"Hello \0World"[..], "Hello "),
      (&b"\0"[..], ""),
    ];

    for (buf, expected) in cases {
      let buf = buf.as_ptr() as *const c_char;
      let actual = buf.into_string();

      assert_eq!(actual, expected);
    }
  }

  #[test]
  #[cfg(windows)]
  fn test_into_str_from_wide_ptr() {
    let cases: Vec<(&[u16], &str)> = vec![
      (wchar::wch!("Hello World\0"), "Hello World"),
      (wchar::wch!("Hello \0World"), "Hello "),
      (wchar::wch!("\0"), ""),
    ];

    for (buf, expected) in cases {
      let buf = buf.as_ptr();
      let actual = buf.into_string();

      assert_eq!(actual, expected);
    }
  }
}
