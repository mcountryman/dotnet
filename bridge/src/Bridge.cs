using System;
using System.Linq;
using System.Runtime.InteropServices;
using Dotnet.Bridge;

public class Bridge
{
  public delegate BridgeContextHandle GetContextHandleFn();

  public static BridgeContextHandle GetContextHandle()
  {
    return BridgeContext.GetHandle();
  }
}

public class BridgeContext
{
  public static unsafe ClrObject Add(byte* buf, int argc)
  {
    var args = ClrObject.From(buf, argc);
    var a = (string)args[0].Value;
    var b = (string)args[1].Value;
    var c = a + b;

    return ClrObject.From(11);
  }

  public static BridgeContextHandle GetHandle()
  {
    var apiType = typeof(BridgeContext);

    var add = apiType.GetMethod(nameof(Add));
    var addHandle = add.MethodHandle.GetFunctionPointer();

    return new BridgeContextHandle
    {
      Add = addHandle,
    };
  }
}

[StructLayout(LayoutKind.Sequential)]
public struct BridgeContextHandle
{
  public IntPtr Add;
}