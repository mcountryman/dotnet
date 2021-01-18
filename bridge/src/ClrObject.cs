using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge
{
  public enum ClrType : uint
  {
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

  [StructLayout(LayoutKind.Explicit, Size = 24)]
  public struct ClrObject
  {
    [FieldOffset(0)]
    [MarshalAs(UnmanagedType.U4)]
    public ClrType Type;
    [MarshalAs(UnmanagedType.U1)]
    [FieldOffset(8)]
    char Char;
    [MarshalAs(UnmanagedType.U1)]
    [FieldOffset(8)]
    byte Byte;
    [MarshalAs(UnmanagedType.I1)]
    [FieldOffset(8)]
    sbyte SByte;
    [MarshalAs(UnmanagedType.Bool)]
    [FieldOffset(8)]
    bool Boolean;
    [MarshalAs(UnmanagedType.I2)]
    [FieldOffset(8)]
    short Short;
    [MarshalAs(UnmanagedType.U2)]
    [FieldOffset(8)]
    ushort UShort;
    [MarshalAs(UnmanagedType.I4)]
    [FieldOffset(8)]
    int Int32;
    [MarshalAs(UnmanagedType.U4)]
    [FieldOffset(8)]
    uint UInt32;
    [MarshalAs(UnmanagedType.I8)]
    [FieldOffset(8)]
    long Int64;
    [MarshalAs(UnmanagedType.U8)]
    [FieldOffset(8)]
    ulong UInt64;
    [MarshalAs(UnmanagedType.R4)]
    [FieldOffset(8)]
    float Float;
    [MarshalAs(UnmanagedType.R8)]
    [FieldOffset(8)]
    double Double;
    [MarshalAs(UnmanagedType.R8)]
    [FieldOffset(8)]
    decimal Decimal;
    [FieldOffset(8)]
    ClrString String;

    public dynamic Value
    {
      get
      {
        switch (Type)
        {
          case ClrType.Char: return Char;
          case ClrType.Byte: return Byte;
          case ClrType.SByte: return SByte;
          case ClrType.Boolean: return Boolean;
          case ClrType.Short: return Short;
          case ClrType.UShort: return UShort;
          case ClrType.Int32: return Int32;
          case ClrType.UInt32: return UInt32;
          case ClrType.Int64: return Int64;
          case ClrType.UInt64: return UInt32;
          case ClrType.Float: return Float;
          case ClrType.Double: return Double;
          case ClrType.Decimal: return Decimal;
          case ClrType.String: return String.ToString();
          default:
            throw new InvalidOperationException($"ClrObject has unexpected type `{Type}`");
        }
      }
    }

    public static ClrObject From(int value)
    {
      return new ClrObject
      {
        Type = ClrType.Int32,
        Int32 = value,
      };
    }
  }
}