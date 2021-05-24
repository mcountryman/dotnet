using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge {
  public interface IBridgeResult<T> {
    T Value { get; }
  }

  [StructLayout(LayoutKind.Sequential)]
  public struct PrepareInvokeResult : IBridgeResult<IntPtr> {
    public IntPtr Value { get; init; }

    public static PrepareInvokeResult FromValue(IntPtr value) {
      return new PrepareInvokeResult() { Value = value };
    }

    public static PrepareInvokeResult FromException<T>(Exception ex) {
      // return new PrepareInvokeResult();
      throw ex;
    }
  }
}