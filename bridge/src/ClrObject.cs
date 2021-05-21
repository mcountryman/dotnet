using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge
{
  public enum ClrType : uint
  {
    Null,
    Char = 0,
    Byte,
    SByte,
    Boolean,
    Short,
    UShort,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    Decimal,
    String,
  }

  [StructLayout(LayoutKind.Sequential)]
  public struct ClrString
  {
    [MarshalAs(UnmanagedType.LPUTF8Str)]
    IntPtr Value;
    [MarshalAs(UnmanagedType.U8)]
    ulong Length;

    public override string ToString()
    {
      return Marshal.PtrToStringUTF8(Value, (int)Length);
    }
  }

  public struct ClrObject
  {
    public ClrType Type { get; private init; }
    public object? Value { get; private init; }

    public static ClrObject From(int value)
    {
      return new ClrObject
      {
        Type = ClrType.Int32,
        Value = value,
      };
    }

    public static unsafe ClrObject[] From(byte* value, int count)
    {
      var result = new ClrObject[count];

      for (var i = 0; i < count; i++)
      {
        result[i] = From(value);
        value += 24;
      }

      return result;
    }

    public static unsafe ClrObject From(byte* buf)
    {
      var type = *(ClrType*)buf;
      var value = buf + 8;

      switch (type)
      {
        case ClrType.Char: return new ClrObject { Type = type, Value = *(char*)value };
        case ClrType.Byte: return new ClrObject { Type = type, Value = *(byte*)value };
        case ClrType.SByte: return new ClrObject { Type = type, Value = *(sbyte*)value };
        case ClrType.Boolean: return new ClrObject { Type = type, Value = *(bool*)value };
        case ClrType.Short: return new ClrObject { Type = type, Value = *(short*)value };
        case ClrType.UShort: return new ClrObject { Type = type, Value = *(ushort*)value };
        case ClrType.Int32: return new ClrObject { Type = type, Value = *(int*)value };
        case ClrType.UInt32: return new ClrObject { Type = type, Value = *(uint*)value };
        case ClrType.Int64: return new ClrObject { Type = type, Value = *(long*)value };
        case ClrType.UInt64: return new ClrObject { Type = type, Value = *(ulong*)value };
        case ClrType.Float: return new ClrObject { Type = type, Value = *(float*)value };
        case ClrType.Double: return new ClrObject { Type = type, Value = *(double*)value };
        case ClrType.Decimal: return new ClrObject { Type = type, Value = *(decimal*)value };
        case ClrType.String:
          return new ClrObject
          {
            Type = type,
            Value = Marshal.PtrToStringUTF8(new IntPtr(value), *(int*)(value + IntPtr.Size)),
          };
        default:
          throw new InvalidOperationException($"ClrObject has unexpected type `{type}`");
      }
    }
  }
}