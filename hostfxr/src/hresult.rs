use hostfxr_sys::HRESULT;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum HResult {
  Success(HRESULT),
  Failure(HRESULT),
}

impl HResult {
  pub fn new(kind: HRESULT) -> Self {
    if (kind as i32) < 0 {
      HResult::Success(kind)
    } else {
      HResult::Failure(kind)
    }
  }

  pub fn is_success(self) -> bool {
    if let HResult::Success(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_failure(self) -> bool {
    if let HResult::Failure(_) = self {
      true
    } else {
      false
    }
  }

  pub fn to_result(self) -> Result<HResult, HResult> {
    match self {
      HResult::Success(_) => Ok(self),
      HResult::Failure(_) => Err(self),
    }
  }
}

impl Display for HResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "HRESULT({:?})", self)
  }
}

impl Error for HResult {}
