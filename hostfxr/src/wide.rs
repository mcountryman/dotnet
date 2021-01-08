/// Lightweight wide/short null suffix string conversion.
use std::borrow::Cow;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct WideString<'a>(Cow<'a, str>);

pub trait IntoWideString<'a> {
  fn into_wide(self) -> WideString<'a>;
}

impl<'a> Into<Vec<u16>> for WideString<'a> {
  fn into(self) -> Vec<u16> {
    let mut bytes = Vec::with_capacity(self.0.len() + 1);
    bytes.extend(self.0.encode_utf16());
    bytes
  }
}

impl<'a> Into<Vec<u8>> for WideString<'a> {
  fn into(self) -> Vec<u8> {
    CString::new(self)
      .expect("CString::new failed to append 0 byte")
      .into_bytes()
  }
}

impl<'a> Into<Vec<u16>> for &'a WideString<'a> {
  fn into(self) -> Vec<u16> {
    let mut bytes = Vec::with_capacity(self.0.len() + 1);
    bytes.extend(self.0.encode_utf16());
    bytes
  }
}

impl<'a> Into<Vec<u8>> for &'a WideString<'a> {
  fn into(self) -> Vec<u8> {
    CString::new(self)
      .expect("CString::new failed to append 0 byte")
      .into_bytes()
  }
}

impl<'a, C: Into<Cow<'a, str>>> From<C> for WideString<'a> {
  fn from(inner: C) -> Self {
    Self(inner.into())
  }
}

impl<'a> IntoWideString<'a> for &'a str {
  fn into_wide(self) -> WideString<'a> {
    WideString(self.into())
  }
}

impl<'a> IntoWideString<'a> for &'a &'a str {
  fn into_wide(self) -> WideString<'a> {
    WideString((*self).into())
  }
}

impl<'a> IntoWideString<'a> for String {
  fn into_wide(self) -> WideString<'a> {
    WideString(self.into())
  }
}

macro_rules! into_wide_args_ptr {
  ($name: ident) => {{
    use std::convert::TryFrom;
    #[allow(unused_imports)]
    use $crate::wide::IntoWideString;

    let argv: Vec<Vec<_>> = $name
      .into_iter()
      .map(|value| value.into_wide().into())
      .collect();
    let mut argv: Vec<*const _> = argv.iter().map(|value| value.as_ptr()).collect();

    (i32::try_from(argv.len())?, argv.as_mut_ptr())
  }};
}

macro_rules! into_wide_ptr {
  ($name: ident) => {{
    #[allow(unused_imports)]
    use $crate::wide::IntoWideString;

    let wide = $name.into_wide();
    let wide: Vec<_> = wide.into();

    wide.as_ptr()
  }};
}
