using System;
using System.Linq;
using System.Runtime.InteropServices;
using System.Reflection;
using System.Collections.Generic;
using Dotnet.Bridge;
using Dotnet.Bridge.Utilities;


[StructLayout(LayoutKind.Sequential)]
public unsafe class Bridge {
  [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
  public delegate BridgeResult<IntPtr> PrepareInvokeDelegate(
    string path,
    BridgeObjectType retType,
    BridgeObjectType[] types
  );

  public PrepareInvokeDelegate PrepareInvoke = (path, retType, types) => {
    try {
      var method = InvokeBuilder.Prepare(path, retType, types);
      return BridgeResult.FromValue(IntPtr.Zero);
    } catch (Exception ex) {
      return BridgeResult.FromException<IntPtr>(ex);
    }
  };


  [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
  public delegate Bridge GetBridgeDelegate();

  public static Bridge GetBridge() {
    return new Bridge();
  }
}