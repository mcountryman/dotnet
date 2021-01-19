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
  public static unsafe ClrObject Add(ClrObject* args, int argc)
  {
    var a = args[0].Value;
    var b = args[1].Value;
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