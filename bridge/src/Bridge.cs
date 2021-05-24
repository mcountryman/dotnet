using System;
using System.Linq;
using System.Runtime.InteropServices;
using System.Reflection;
using System.Collections.Generic;
using Dotnet.Bridge;
using Dotnet.Bridge.Utilities;
using System.Security.Permissions;

[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
public delegate Bridge GetBridgeDelegate();

[UnmanagedFunctionPointer(CallingConvention.StdCall)]
public unsafe delegate PrepareInvokeResult PrepareInvokeDelegate(
  [MarshalAs(UnmanagedType.LPUTF8Str)]
  string path,
  BridgeObjectType retType,
  BridgeObjectType* types,
  ushort typesLength
);

[StructLayout(LayoutKind.Sequential)]
public unsafe ref struct Bridge {
  public IntPtr PrepareInvoke;

  public static int Add(int a, int b) {
    return a + b;
  }

  public static PrepareInvokeResult PrepareInvokeImp(
    string path,
    BridgeObjectType retType,
    BridgeObjectType* typesPtr,
    ushort typesLength
  ) {
    Console.WriteLine($"PrepareInvokeImp({(byte)retType})");

    var types = new BridgeObjectType[typesLength];
    for (var i = 0; i < types.Length; i++) {
      types[i] = *typesPtr;
      typesPtr++;
    }

    // try {
    var method = InvokeBuilder.Prepare(path, retType, types);
    return PrepareInvokeResult.FromValue(method);
    // } catch (Exception ex) {
    //   return PrepareInvokeResult.FromException<IntPtr>(ex);
    // }
  }


  public static Bridge GetBridge() {
    return new Bridge {
      PrepareInvoke = Marshal.GetFunctionPointerForDelegate<PrepareInvokeDelegate>(PrepareInvokeImp),
    };
  }
}
