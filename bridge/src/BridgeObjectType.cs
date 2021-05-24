using System;

namespace Dotnet.Bridge {
  public enum BridgeObjectType : uint {
    Char,
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
    Object,
  }

  public static class BridgeObjectTypeEx {
    public static Type AsType(this BridgeObjectType type) {
      return type switch {
        BridgeObjectType.Char => typeof(char),
        BridgeObjectType.Byte => typeof(byte),
        BridgeObjectType.SByte => typeof(sbyte),
        BridgeObjectType.Boolean => typeof(bool),
        BridgeObjectType.Short => typeof(short),
        BridgeObjectType.UShort => typeof(ushort),
        BridgeObjectType.Int32 => typeof(int),
        BridgeObjectType.UInt32 => typeof(uint),
        BridgeObjectType.Int64 => typeof(long),
        BridgeObjectType.UInt64 => typeof(ulong),
        BridgeObjectType.Float => typeof(float),
        BridgeObjectType.Double => typeof(double),
        BridgeObjectType.Decimal => typeof(decimal),
        BridgeObjectType.String => typeof(string),
        BridgeObjectType.Object => typeof(object),
      };
    }
  }
}