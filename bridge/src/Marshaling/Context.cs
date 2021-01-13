
using System;

namespace Dotnet.Bridge.Marshaling {
  public static class MarshalContext
  {
    public delegate T MapToFn<F, T>(F from);
    public delegate dynamic MarshalFn(object from);
    public delegate dynamic MarshalTypedFn<F>(F from);

    public static void MapTo<F, T>(F from, MapToFn<F, T> map) { throw new NotImplementedException(); }
    public static IMarshaler? Resolve(Type from) { throw new NotImplementedException(); }
    public static IMarshaler? Resolve<F>() { throw new NotImplementedException(); }
  }
}
