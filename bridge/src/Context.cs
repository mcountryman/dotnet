using System;
using System.Runtime.InteropServices;

public interface IBridgeContext
{
  void GetMethod(string assemblyQualifiedNameOrSignature);
  void GetField(string assemblyQualifiedName);
  void SetField(string assemblyQualiedName, object value);
}

[StructLayout(LayoutKind.Sequential)]
public struct BridgeContextHandle
{
  public IntPtr GetMethodHandle;
  public IntPtr GetFieldHandle;
  public IntPtr SetFieldHandle;
}

public static class IBridgeContextEx
{
  public static BridgeContextHandle GetHandle(this IBridgeContext api)
  {
    var apiType = api.GetType();

    var getMethod = apiType.GetMethod(nameof(IBridgeContext.GetMethod));
    var getMethodHandle = Marshal.GetFunctionPointerForDelegate(getMethod);

    var getField = apiType.GetMethod(nameof(IBridgeContext.GetField));
    var getFieldHandle = Marshal.GetFunctionPointerForDelegate(getField);

    var setField = apiType.GetMethod(nameof(IBridgeContext.SetField));
    var setFieldHandle = Marshal.GetFunctionPointerForDelegate(setField);

    return new BridgeContextHandle { 
      GetFieldHandle = getFieldHandle,
      SetFieldHandle = setFieldHandle,
      GetMethodHandle = getMethodHandle,
    };
  }
}
