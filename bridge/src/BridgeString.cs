using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge {

  [StructLayout(LayoutKind.Sequential)]
  public struct BridgeString {
    [MarshalAs(UnmanagedType.LPUTF8Str)]
    public IntPtr Value;
    [MarshalAs(UnmanagedType.U8)]
    public ulong Length;

    public override string ToString() {
      return Marshal.PtrToStringUTF8(Value, (int)Length);
    }
  }
}