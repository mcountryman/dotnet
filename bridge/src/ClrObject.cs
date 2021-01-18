using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge
{
  public enum ClrType : byte
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
  }

  [StructLayout(LayoutKind.Sequential)]
  public struct ClrString
  {
    [MarshalAs(UnmanagedType.LPUTF8Str)]
    IntPtr Value;
    [MarshalAs(UnmanagedType.U8)]
    ulong Length;
  }

  [StructLayout(LayoutKind.Explicit)]
  public struct ClrObject
  {
    [MarshalAs(UnmanagedType.U2)]
    [FieldOffset(0)]
    ClrType Type;
    [MarshalAs(UnmanagedType.U2)]
    [FieldOffset(1)]
    char Char;
    [MarshalAs(UnmanagedType.U2)]
    [FieldOffset(1)]
    byte Byte;
    [MarshalAs(UnmanagedType.I2)]
    [FieldOffset(1)]
    sbyte SByte;
    [MarshalAs(UnmanagedType.Bool)]
    [FieldOffset(1)]
    bool Boolean;
    [MarshalAs(UnmanagedType.I4)]
    [FieldOffset(1)]
    short Short;
    [MarshalAs(UnmanagedType.U4)]
    [FieldOffset(1)]
    ushort UShort;
    [MarshalAs(UnmanagedType.I4)]
    [FieldOffset(1)]
    int Int32;
    [MarshalAs(UnmanagedType.U4)]
    [FieldOffset(1)]
    uint UInt32;
    [MarshalAs(UnmanagedType.I8)]
    [FieldOffset(1)]
    long Int64;
    [MarshalAs(UnmanagedType.U8)]
    [FieldOffset(1)]
    ulong UInt64;
    [MarshalAs(UnmanagedType.R4)]
    [FieldOffset(1)]
    float Float;
    [MarshalAs(UnmanagedType.R4)]
    [FieldOffset(1)]
    double Double;
    [MarshalAs(UnmanagedType.R8)]
    [FieldOffset(1)]
    decimal Decimal;
    [FieldOffset(1)]
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