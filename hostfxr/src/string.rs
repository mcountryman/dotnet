/// Lightweight wide/short null suffix string conversion.
use std::borrow::Cow;
use std::ffi::OsString;
use std::os::raw::c_char;
use std::slice::from_raw_parts;

#[derive(Debug, Clone)]
pub struct WideString<'a>(Cow<'a, str>);

pub trait IntoBytes<T> {
  fn into_bytes(self) -> Vec<T>;
}

pub trait IntoStr<'a> {
  fn into_str(self) -> Cow<'a, str>;
}

impl<'a> IntoStr<'a> for &'a [c_char] {
  fn into_str(self) -> Cow<'a, str> {
    let len = self.len();
    let buf = self.as_ptr();
    let buf = unsafe { from_raw_parts(buf as *const _, len) };
    let buf = String::from_utf8_lossy(buf).to_string();

    Cow::from(buf)
  }
}

impl<'a> IntoStr<'a> for *const c_char {
  fn into_str(self) -> Cow<'a, str> {
    unimplemented!()
  }
}

#[cfg(windows)]
impl<'a> IntoStr<'a> for &'a [u16] {
  fn into_str(self) -> Cow<'a, str> {
    use std::os::windows::prelude::*;

    let buf = OsString::from_wide(self);
    let buf = buf.to_string_lossy();
    let buf = buf.to_string();

    Cow::from(buf)
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
    let buf = OsString::from(buf);
    let buf: Vec<_> = buf.encode_wide().chain(Some(0)).collect();

    buf
  }
}

#[macro_export]
macro_rules! into_args {
  ($args: ident) => {{
    #[allow(unused_imports)]
    use $crate::string::IntoBytes;

    let args: Vec<Vec<_>> = $args.into_iter().map(|arg| arg.into_bytes()).collect();
    let mut args: Vec<*const _> = args.into_iter().map(|arg| arg.as_ptr()).collect();

    let argv = args.as_mut_ptr();
    let argc = args.len() as _;

    (argv, argc)
  }};
}

#[cfg(test)]
mod tests {
  use std::slice;

  use super::*;

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

      let actual = buf.into_str();
      assert_eq!(actual, expected);
    }
  }

  #[test]
  fn test_into_str_from_wide() {
    let cases: Vec<(&[u16], &str)> = vec![
      (wchar::wch!("Hello World"), "Hello World"),
      (wchar::wch!("Hello World\0"), "Hello World\u{0}"),
      (wchar::wch!("Hello \0World"), "Hello \u{0}World"),
      (wchar::wch!("\0"), "\u{0}"),
      (wchar::wch!(""), ""),
    ];

    for (buf, expected) in cases {
      let actual = buf.into_str();
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
}
// #[cfg(windows)]
// impl<'a, C: Into<Cow<'a, [u16]>>> IntoOsString for C {
//   fn into_os_str(self) -> OsString {
//     OsString::from_wide(&self.into())
//   }
// }

// #[cfg(windows)]
// impl<'a> Into<Vec<u16>> for OsString {
//   fn into(self) -> Vec<u16> {
//     self.to_wide_null()
//   }
// }
//
// impl<'a> Into<Vec<u8>> for OsString {
//   fn into(self) -> Vec<u8> {
//     self.as_bytes().into_iter().chain(Some(&0)).collect()
//   }
// }
