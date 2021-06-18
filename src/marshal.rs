use crate::types::TypeId;
use std::{mem::ManuallyDrop, slice::from_raw_parts};

#[derive(thiserror::Error, Debug)]
pub enum MarshalError {
  #[error(transparent)]
  Custom(#[from] Box<dyn std::error::Error>),
}

pub trait Marshal {
  type Managed: Sized;

  fn id() -> TypeId;
  fn blittable() -> bool {
    false
  }
}

impl<M: Marshal> Marshal for &M {
  type Managed = M::Managed;

  fn id() -> TypeId {
    M::id()
  }
}

pub trait MarshalTo: Marshal {
  fn marshal_to(self) -> Result<Self::Managed, MarshalError>;
}

pub trait MarshalFrom: Marshal + Sized {
  fn marshal_from(from: Self::Managed) -> Result<Self, MarshalError>;
}

macro_rules! marshal_blittable {
  ($type:ty, $type_id:expr) => {
    impl Marshal for $type {
      type Managed = $type;

      fn id() -> TypeId {
        $type_id
      }

      fn blittable() -> bool {
        true
      }
    }

    impl MarshalTo for $type {
      #[inline]
      fn marshal_to(self) -> Result<Self::Managed, MarshalError> {
        Ok(self)
      }
    }

    impl MarshalFrom for $type {
      #[inline]
      fn marshal_from(from: Self::Managed) -> Result<Self, MarshalError> {
        Ok(from)
      }
    }
  };
}

marshal_blittable!(bool, TypeId::Boolean);

marshal_blittable!(u8, TypeId::Byte);
marshal_blittable!(u16, TypeId::UInt16);
marshal_blittable!(u32, TypeId::UInt32);
marshal_blittable!(u64, TypeId::UInt64);

marshal_blittable!(i8, TypeId::SByte);
marshal_blittable!(i16, TypeId::Int16);
marshal_blittable!(i32, TypeId::Int32);
marshal_blittable!(i64, TypeId::Int64);

marshal_blittable!(f32, TypeId::Float);
marshal_blittable!(f64, TypeId::Double);

#[cfg(target_pointer_width = "32")]
marshal_blittable!(usize, TypeId::UInt32);
#[cfg(target_pointer_width = "64")]
marshal_blittable!(usize, TypeId::UInt64);

impl<M: Marshal> Marshal for &[M] {
  type Managed = (*const M::Managed, u32);

  fn id() -> TypeId {
    TypeId::Array(Box::new(M::id()))
  }
}

impl<M> MarshalTo for &[M]
where
  M: MarshalTo + Clone,
{
  fn marshal_to(self) -> Result<Self::Managed, MarshalError> {
    let vec = self
      .iter()
      .cloned()
      .map(|val| val.marshal_to())
      .collect::<Result<Vec<_>, _>>()?;

    let vec = ManuallyDrop::new(vec);
    let len = vec.len();
    let ptr = vec.as_ptr() as *const _;

    Ok((ptr, len as _))
  }
}

impl<M: Marshal> Marshal for Vec<M> {
  type Managed = (*mut M::Managed, u32);

  fn id() -> TypeId {
    TypeId::Array(Box::new(M::id()))
  }
}

impl<M> MarshalFrom for Vec<M>
where
  M: MarshalFrom,
  M::Managed: Clone,
{
  fn marshal_from(from: Self::Managed) -> Result<Self, MarshalError> {
    unsafe { from_raw_parts(from.0, from.1 as _) }
      .iter()
      .cloned()
      .map(M::marshal_from)
      .collect::<Result<Vec<_>, _>>()
  }
}

impl Marshal for String {
  type Managed = (*const u8, u32);

  fn id() -> TypeId {
    TypeId::String
  }
}

impl MarshalFrom for String {
  fn marshal_from(from: Self::Managed) -> Result<Self, MarshalError> {
    Ok(
      String::from_utf8_lossy(unsafe { from_raw_parts(from.0, from.1 as _) }).to_string(),
    )
  }
}
