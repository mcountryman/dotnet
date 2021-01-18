// using System;
// using System.Runtime.InteropServices;

// [StructLayout(LayoutKind.Sequential)]
// public struct ClrObject
// {
//   public int Type;
//   public string Value;
// }

// public interface IBridgeContext
// {
//   int AddStd(int a, int b);
//   unsafe ClrObject AddDyn(ClrObject* argv, uint argc);
// }


// public static class IBridgeContextEx
// {
//   public static BridgeContextHandle GetHandle(this IBridgeContext api)
//   {
//     var apiType = api.GetType();

//     var addStd = apiType.GetMethod(nameof(IBridgeContext.AddStd));
//     var addStdHandle = addStd.MethodHandle.GetFunctionPointer();

//     var addDyn = apiType.GetMethod(nameof(IBridgeContext.AddDyn));
//     var addDynHandle = addDyn.MethodHandle.GetFunctionPointer();

//     return new BridgeContextHandle
//     {
//       AddStd = addStdHandle,
//       AddDyn = addDynHandle,
//     };
//   }
// }
