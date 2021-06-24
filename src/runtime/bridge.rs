use crate::{types::TypeId, Runtime};
use std::{
  borrow::Cow, ffi::c_void, marker::PhantomData, mem::ManuallyDrop, ptr::NonNull,
};

/// Get bridge `GetBridge` method assembly qualified type name
pub fn get_bridge_type_name() -> &'static str {
  "Bridge, Bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null"
}

/// Get bridge `GetBridge` method name
pub fn get_bridge_method_name() -> &'static str {
  "GetBridge"
}

/// Get bridge `GetBridge` method assembly qualified delegate type name
pub fn get_bridge_delegate_name() -> &'static str {
  "GetBridgeDelegate, Bridge, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null"
}

/// Get path to bridge assembly
pub fn get_bridge_assembly_path() -> Cow<'static, str> {
  cfg_if::cfg_if! {
    if #[cfg(debug_assertions)] {
      "bridge/bin/Release/net5.0/bridge.dll".into()
    } else {
      compile_error!("Runtime assembly dropping in %TEMP% not implemented")
    }
  }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum BridgeError {
  #[error("Method not found")]
  MethodNotFound,
}

#[derive(Clone)]
pub struct Bridge<'rt, R: Runtime> {
  imp: Box<ffi::BridgeImpl>,
  phantom: PhantomData<&'rt R>,
}

impl<'rt, R: Runtime> Bridge<'rt, R> {
  /// Construct bridge from supplied handle
  ///
  /// # Safety
  /// Assumes that handle points to a struct that represents [`BridgeImpl`]
  pub unsafe fn from_handle(handle: *mut c_void) -> Option<Self> {
    let imp = NonNull::new(handle as *mut ffi::BridgeImpl);
    let imp = match imp {
      Some(imp) => Box::from_raw(imp.as_ptr()),
      None => return None,
    };

    println!("test: {}", imp.test);

    Some(Self {
      imp,
      phantom: Default::default(),
    })
  }

  pub fn get_method<F: Sized>(
    &self,
    path: &str,
    types: Vec<TypeId>,
  ) -> Result<&F, BridgeError> {
    let mut path = ManuallyDrop::new(path.to_string());
    let mut types = ManuallyDrop::new(types);

    unsafe {
      (*self.imp.get_method)(
        path.as_mut_ptr(),
        path.len() as _,
        types.as_mut_ptr(),
        types.len() as _,
      )
      .into_result()
      .map(|ptr| &*(ptr as *const *mut () as *const F))
    }
  }

  pub fn release<T>(&self, handle: &mut NonNull<T>) -> Result<(), BridgeError> {
    println!("release(..)");
    println!("result: {}", unsafe { (*self.imp.release)(0) });

    Ok(())
  }
}

mod ffi {
  use super::BridgeError;
  use crate::types::TypeId;

  pub type ReleaseFn = unsafe extern "stdcall" fn(handle: usize) -> usize;
  pub type GetMethodFn = unsafe extern "stdcall" fn(
    path: *mut u8,
    path_len: u32,
    types: *mut TypeId,
    types_len: u16,
  ) -> BridgeResult<*const ()>;

  #[repr(C)]
  #[derive(Clone)]
  pub struct BridgeImpl {
    pub release: *mut ReleaseFn,
    pub get_method: *mut GetMethodFn,
    pub test: i32,
  }

  unsafe impl Send for BridgeImpl {}
  unsafe impl Sync for BridgeImpl {}

  #[repr(C, u8)]
  #[allow(dead_code)]
  pub enum BridgeResult<T> {
    Ok(T),
    Err(BridgeError),
  }

  impl<T> BridgeResult<T> {
    pub fn into_result(self) -> Result<T, BridgeError> {
      match self {
        Self::Ok(val) => Ok(val),
        Self::Err(err) => Err(err),
      }
    }
  }
}
