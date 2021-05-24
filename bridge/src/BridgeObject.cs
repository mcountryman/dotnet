using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge {
  [StructLayout(LayoutKind.Sequential)]
  public class BridgeObject {
    [MarshalAs(UnmanagedType.I8)]
    public BridgeObjectType Type;
    [MarshalAs(UnmanagedType.LPStruct)]
    public object? Value;

    public static BridgeObject From(int value) {
      return new BridgeObject {
        Type = BridgeObjectType.Int32,
        Value = value,
      };
    }

    public static unsafe BridgeObject[] From(byte* value, ushort count) {
      var result = new BridgeObject[count];

      for (var i = 0; i < count; i++) {
        result[i] = From(value);
        value += 24;
      }

      return result;
    }

    public static unsafe BridgeObject From(byte* buf) {
      var type = *(BridgeObjectType*)buf;
      var value = buf + 8;

      switch (type) {
        case BridgeObjectType.Char: return new BridgeObject { Type = type, Value = *(char*)value };
        case BridgeObjectType.Byte: return new BridgeObject { Type = type, Value = *(byte*)value };
        case BridgeObjectType.SByte: return new BridgeObject { Type = type, Value = *(sbyte*)value };
        case BridgeObjectType.Boolean: return new BridgeObject { Type = type, Value = *(bool*)value };
        case BridgeObjectType.Short: return new BridgeObject { Type = type, Value = *(short*)value };
        case BridgeObjectType.UShort: return new BridgeObject { Type = type, Value = *(ushort*)value };
        case BridgeObjectType.Int32: return new BridgeObject { Type = type, Value = *(int*)value };
        case BridgeObjectType.UInt32: return new BridgeObject { Type = type, Value = *(uint*)value };
        case BridgeObjectType.Int64: return new BridgeObject { Type = type, Value = *(long*)value };
        case BridgeObjectType.UInt64: return new BridgeObject { Type = type, Value = *(ulong*)value };
        case BridgeObjectType.Float: return new BridgeObject { Type = type, Value = *(float*)value };
        case BridgeObjectType.Double: return new BridgeObject { Type = type, Value = *(double*)value };
        case BridgeObjectType.Decimal: return new BridgeObject { Type = type, Value = *(decimal*)value };
        case BridgeObjectType.String:
          return new BridgeObject {
            Type = type,
            Value = Marshal.PtrToStringUTF8(new IntPtr(value), *(int*)(value + IntPtr.Size)),
          };
        default:
          throw new InvalidOperationException($"Unexpected type `{type}`");
      }
    }

    public static BridgeObject From(object? obj) {
      return obj switch {
        bool value => new BridgeObject { Type = BridgeObjectType.Boolean, Value = value },
        char value => new BridgeObject { Type = BridgeObjectType.Char, Value = value },
        byte value => new BridgeObject { Type = BridgeObjectType.Byte, Value = value },
        sbyte value => new BridgeObject { Type = BridgeObjectType.SByte, Value = value },
        int value => new BridgeObject { Type = BridgeObjectType.Int32, Value = value },
        uint value => new BridgeObject { Type = BridgeObjectType.UInt32, Value = value },
        long value => new BridgeObject { Type = BridgeObjectType.UInt64, Value = value },
        ulong value => new BridgeObject { Type = BridgeObjectType.UInt64, Value = value },
        float value => new BridgeObject { Type = BridgeObjectType.Float, Value = value },
        double value => new BridgeObject { Type = BridgeObjectType.Double, Value = value },
        decimal value => new BridgeObject { Type = BridgeObjectType.Decimal, Value = value },
        string value => new BridgeObject { Type = BridgeObjectType.String, Value = Marshal.StringToBSTR(value) },
        null => new BridgeObject { Type = BridgeObjectType.Object, Value = null },
        _ => throw new InvalidOperationException("Unsupported type"),
      };
    }

    public static BridgeObject Null = new BridgeObject {
      Type = BridgeObjectType.Object,
      Value = null,
    };
  }
}