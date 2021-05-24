using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge {

  [StructLayout(LayoutKind.Sequential)]
  public struct BridgeResult<T> {
    [MarshalAs(UnmanagedType.LPStruct)]
    public T Value;
  }

  public class BridgeResult {
    public static BridgeResult<T> FromValue<T>(T value) {
      return new BridgeResult<T>();
    }

    public static BridgeResult<T> FromException<T>(Exception ex) {
      return new BridgeResult<T>();
    }
  }
}