pub trait Method<Args = ()> {
  type Return;

  fn invoke(&self, args: Args) -> Self::Return;
}

impl<F, Ret, A, B, C> Method<(A, B, C)> for F
where
  F: Fn(&A, &B, &C) -> Ret,
{
  type Return = Ret;

  fn invoke(&self, args: (A, B, C)) -> Self::Return {
    // #[allow(non_snake_case)]
    // let ($($arg,)*) = args;
    // (self)($($arg,)*)
    todo!()
  }
}

// macro_rules! method_impl {
//   ($($arg:ident)*) => {
//     impl<F, Ret, $($arg,)*> Method<($($arg,)*)> for F
//     where
//       F: Fn($($arg,)*) -> Ret,
//     {
//       type Return = Ret;

//       fn invoke(&self, args: ($($arg,)*)) -> Self::Return {
//         // #[allow(non_snake_case)]
//         // let ($($arg,)*) = args;
//         // (self)($($arg,)*)
//         todo!()
//       }
//     }
//   };
// }

// method_impl! { A }
// method_impl! { A, B }
// method_impl! { A, B, C }
// method_impl! { A, B, C, D }
// method_impl! { A, B, C, D, E }
// method_impl! { A, B, C, D, E, F }
// method_impl! { A, B, C, D, E, F, G }
