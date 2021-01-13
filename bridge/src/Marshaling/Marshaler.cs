#nullable enable

using System;
using System.Linq;
using System.Runtime.CompilerServices;

namespace Dotnet.Bridge.Marshaling
{
  public interface IMarshaler
  {
    Type[] NativeTypes { get; }
    Type ManagedType { get; }

    object[] MarshalTo(object from);
    object MarshalFrom(object[] from);
  }

  public abstract class Marshaler<TManaged, TNative> : IMarshaler
  {
    public Type[] NativeTypes => new[] { typeof(TNative) };
    public Type ManagedType => typeof(TManaged);

    public object[] MarshalTo(object from)
    {
      return new[] { (object)MarshalTo((TManaged)from)! }!;
    }

    public object MarshalFrom(object[] from)
    {
      if (from.Length == 0)
      {
        throw new ArgumentOutOfRangeException(nameof(from));
      }

      return (object)MarshalFrom((TNative)from[0])!;
    }

    public abstract TNative MarshalTo(TManaged from);
    public abstract TManaged MarshalFrom(TNative from);
  }

  public abstract class TupleMarshaller<TManaged, TNative> : IMarshaler
    where TNative : ITuple
  {
    public Type[] NativeTypes => _nativeTypes;
    public Type ManagedType => typeof(TManaged);

    public object[] MarshalTo(object from)
    {
      var native = MarshalTo((TManaged)from);
      return Enumerable.Range(0, native.Length)
        .Select(i => native[i])
        .ToArray()!;
    }

    public abstract object MarshalFrom(object[] from);
    public abstract TNative MarshalTo(TManaged from);

    private static readonly Type[] _nativeTypes = typeof(TNative)
      .GetGenericArguments()
      .ToArray();
  }

  public abstract class TupleMarshaller<TManaged, T1, T2> : TupleMarshaller<TManaged, (T1, T2)>
  {
    public abstract TManaged MarshalFrom(T1 a, T2 b);

    public override object MarshalFrom(object[] from)
    {
      if (from.Length != 2)
      {
        throw new ArgumentOutOfRangeException(nameof(from));
      }

      return (object)MarshalFrom((T1)from[0], (T2)from[1])!;
    }
  }

  public abstract class TupleMarshaller<TManaged, T1, T2, T3> : TupleMarshaller<TManaged, (T1, T2, T3)>
  {
    public abstract TManaged MarshalFrom(T1 a, T2 b, T3 c);

    public override object MarshalFrom(object[] from)
    {
      if (from.Length != 3)
      {
        throw new ArgumentOutOfRangeException(nameof(from));
      }

      return (object)MarshalFrom((T1)from[0], (T2)from[1], (T3)from[2])!;
    }
  }
}
