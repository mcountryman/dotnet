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
    Console.Write("arg: [");
    Console.Write(
      String.Join(", ",
        Enumerable.Range(0, argc)
          .Select(i => new { Type = args[i].Type, Value = args[i].Value }.ToString())
          .ToArray()
      )
    );

    Console.Write("]\n");

    return ClrObject.From((int)0);
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