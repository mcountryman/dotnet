using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge.Marshaling
{
  public class StringMarshaler : Marshaler<string, IntPtr>
  {
    public override IntPtr MarshalTo(string from)
    {
      return Marshal.StringToHGlobalUni(from);
    }

    public override string MarshalFrom(IntPtr ptr)
    {
      return Marshal.PtrToStringUTF8(ptr)!;
    }
  }
}