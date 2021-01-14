using System;
using System.Linq;
using System.Runtime.CompilerServices;

namespace Dotnet.Bridge.Marshaling
{
  public interface IMarshaler
  {
    Type NativeType { get; }
    Type ManagedType { get; }

    object MarshalTo(object from);
    object MarshalFrom(object from);
  }

  public abstract class Marshaler<TManaged, TNative> : IMarshaler
  {
    public Type NativeType => typeof(TNative);
    public Type ManagedType => typeof(TManaged);

    public object MarshalTo(object from)
    {
      return (object)MarshalTo((TManaged)from)!;
    }

    public object MarshalFrom(object from)
    {
      return (object)MarshalFrom((TNative)from)!;
    }

    public abstract TNative MarshalTo(TManaged from);
    public abstract TManaged MarshalFrom(TNative from);
  }
}
