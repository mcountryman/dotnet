using System;
using System.Runtime.InteropServices;

[StructLayout(LayoutKind.Sequential)]
public unsafe struct Bridge {
  IntPtr Release;
  IntPtr GetMethod;
  int Test;

  public static BridgeResult ReleaseImp(IntPtr handle) {
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

    var release = Marshal.GetFunctionPointerForDelegate((ReleaseDelegate)ReleaseImp);
    Console.WriteLine("release: 0x{0:X}", release);

    var bridge = new Bridge {
      Release = release,
      GetMethod = Marshal.GetFunctionPointerForDelegate((GetMethodDelegate)GetMethodImp),
      Test = 69420
    };

    var handle = GCHandle.Alloc(bridge, GCHandleType.Pinned);

    Console.WriteLine("handle: 0x{0:X}", (IntPtr)handle);

    return (IntPtr)handle;
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

[UnmanagedFunctionPointer(CallingConvention.StdCall)]
public unsafe delegate int GetMethodDelegate(
  [MarshalAs(UnmanagedType.LPUTF8Str)]
  string path,
  int* types,
  ushort typesLength
);

[UnmanagedFunctionPointer(CallingConvention.StdCall)]
public unsafe delegate BridgeResult ReleaseDelegate(IntPtr handle);