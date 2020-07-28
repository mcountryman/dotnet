using System;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Dotnet {
  [StructLayout(LayoutKind.Sequential)]
  public struct RuntimeMethods {
    public IntPtr GetTypeHandle;
    public IntPtr GetMethodHandle;
    public IntPtr GetAssemblyHandle;
  }

  public delegate RuntimeMethods GetDelegate();

  public class Runtime {
    public static int Entry(IntPtr z, int a) {
      return 69;
    }

    public static RuntimeMethods Get() {
      return new RuntimeMethods {
        GetTypeHandle = GetMethodPtr(nameof(GetType)),
        GetMethodHandle = GetMethodPtr(nameof(GetMethod)),
        GetAssemblyHandle = GetMethodPtr(nameof(GetAssembly)),
      };
    }

    private static Type GetType(Assembly assembly, string typeName) {
      throw new NotImplementedException();
    }

    private static MethodInfo GetMethod(Type type, string methodName) {
      throw new NotImplementedException();
    }

    private static Assembly GetAssembly(string fileName) {
      throw new NotImplementedException();
    }

    private static IntPtr GetMethodPtr(string methodName) {
      return typeof(Runtime).GetMethod(methodName).MethodHandle.GetFunctionPointer();
    }
  }
}