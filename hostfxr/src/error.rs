use crate::nethost::GetHostFxrError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::TryFromIntError;

pub type HostFxrResult<T> = Result<T, HostFxrError>;

#[derive(Debug)]
pub enum HostFxrError {
  Io(std::io::Error),
  Library(libloading::Error),
  Unexpected(i32),
  ImportNotFound,
  MissingHostPath,
  MissingDotnetRoot,
  TryFromIntError(TryFromIntError),
  ResolveHostFxr(GetHostFxrError),
}

impl From<libloading::Error> for HostFxrError {
  fn from(inner: libloading::Error) -> Self {
    Self::Library(inner)
  }
}

impl From<TryFromIntError> for HostFxrError {
  fn from(inner: TryFromIntError) -> Self {
    Self::TryFromIntError(inner)
  }
}

impl From<GetHostFxrError> for HostFxrError {
  fn from(inner: GetHostFxrError) -> Self {
    Self::ResolveHostFxr(inner)
  }
}

impl Display for HostFxrError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for HostFxrError {}
