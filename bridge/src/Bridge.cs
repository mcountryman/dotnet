using System;
using System.Runtime.InteropServices;

[StructLayout(LayoutKind.Sequential)]
public unsafe struct Bridge {

  [UnmanagedFunctionPointer(CallingConvention.StdCall)]
  public unsafe delegate IntPtr ReleaseDelegate(IntPtr handle);

  [UnmanagedFunctionPointer(CallingConvention.StdCall)]
  public unsafe delegate int GetMethodDelegate(
    [MarshalAs(UnmanagedType.LPUTF8Str)]
  string path,
    int* types,
    ushort typesLength
  );

  IntPtr Release;
  IntPtr GetMethod;
  int Test;

  public static IntPtr ReleaseImp(IntPtr handle) {
    Console.WriteLine("ReleaseImp(IntPtr handle)");

    throw new NotImplementedException();
  }

  public static int GetMethodImp(
    [MarshalAs(UnmanagedType.LPUTF8Str)]
    string path,
    int* types,
    ushort typesLength
  ) {
    Console.WriteLine("GetMethodImp(..)");

    throw new NotImplementedException();
  }

  public static IntPtr GetBridge() {
    Console.WriteLine("GetBridge()");

    var bridge = new Bridge {
      Release = Marshal.GetFunctionPointerForDelegate((ReleaseDelegate)ReleaseImp),
      GetMethod = Marshal.GetFunctionPointerForDelegate((GetMethodDelegate)GetMethodImp),
      Test = 69420
    };

    var handle = Marshal.AllocHGlobal(sizeof(Bridge));
    Marshal.StructureToPtr(bridge, handle, false);
    return handle;
  }
}

public enum BridgeStatus : byte {
  Ok,
  Err
}

public enum BridgeError : byte { }

[StructLayout(LayoutKind.Explicit)]
public struct BridgeResult<T> {
  [FieldOffset(0)]
  public BridgeStatus Status;
  [FieldOffset(1)]
  public T Value;
  [FieldOffset(1)]
  public BridgeError Error;

  public static BridgeResult<T> Ok(T value) {
    return new BridgeResult<T> {
      Status = BridgeStatus.Ok,
      Value = value,
      Error = default!,
    };
  }

  public static BridgeResult<T> Err(BridgeError err) {
    return new BridgeResult<T> {
      Status = BridgeStatus.Err,
      Value = default!,
      Error = err,
    };
  }
}

[StructLayout(LayoutKind.Explicit)]
public struct BridgeResult {
  [FieldOffset(0)]
  public BridgeStatus Status;
  [FieldOffset(1)]
  public IntPtr Value;
  [FieldOffset(1)]
  public BridgeError Error;

  public static BridgeResult Ok() {
    return new BridgeResult {
      Status = BridgeStatus.Ok,
      Value = IntPtr.Zero,
      Error = default!,
    };
  }

  public static BridgeResult Err(BridgeError err) {
    return new BridgeResult {
      Value = IntPtr.Zero,
      Status = BridgeStatus.Err,
      Error = err,
    };
  }
}

[UnmanagedFunctionPointer(CallingConvention.StdCall)]
public delegate IntPtr GetBridgeDelegate();