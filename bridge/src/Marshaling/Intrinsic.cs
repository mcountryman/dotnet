using System;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge.Marshaling
{
  public class StringMarshaller : TupleMarshaller<string, IntPtr, int>
  {
    public override (IntPtr, int) MarshalTo(string from)
    {
      return (Marshal.StringToHGlobalUni(from), from.Length);
    }

    public override string MarshalFrom(IntPtr ptr, int len)
    {
      return Marshal.PtrToStringUTF8(ptr, len);
    }
  }
}