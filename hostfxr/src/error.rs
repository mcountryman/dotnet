use std::num::TryFromIntError;
use std::fmt::{Formatter, Display};
use std::error::Error;

pub type HostFxrResult<T> = Result<T, HostFxrError>;

#[derive(Debug)]
pub enum HostFxrError {
  Library(libloading::Error),
  ImportNotFound,
  MissingHostPath,
  MissingDotnetRoot,
  TryFromIntError(TryFromIntError),
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

impl Display for HostFxrError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for HostFxrError {}