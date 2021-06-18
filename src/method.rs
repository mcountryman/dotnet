use crate::{
  marshal::{MarshalFrom, MarshalTo},
  types::TypeId,
};

pub trait Method<Args> {
  type Fn;
  type Ret;

  fn ret_type_id() -> TypeId;
  fn arg_type_ids() -> Vec<TypeId>;
}

macro_rules! method_impl {
  ($($arg:ident)*) => {
    impl<_F, _R, $($arg),*> Method<($($arg,)*)> for _F
    where
      _F: Fn($($arg),*) -> _R,
      _R: MarshalFrom,
      $($arg: MarshalTo),*
    {
      type Fn = fn($($arg),*) -> _R;
      type Ret = _R;

      fn ret_type_id() -> TypeId {
        _R::id()
      }

      fn arg_type_ids() -> Vec<TypeId> {
        vec![$($arg::id()),*]
      }
    }
  };
}

method_impl! {}
method_impl! { A }
method_impl! { A B }
method_impl! { A B C }
method_impl! { A B C D }
method_impl! { A B C D E }
method_impl! { A B C D E F }
method_impl! { A B C D E F G }
