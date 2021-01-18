// using System;
// using System.Collections.Generic;
// using System.Linq;
// using System.Reflection;
// using System.Reflection.Emit;
// using System.Runtime.InteropServices;
// using Dotnet.Bridge.Marshaling.Utilities;

// namespace Dotnet.Bridge.Marshaling
// {
//   public class MethodInfoFormatter
//   {
//     public static IntPtr Serialize(MethodInfo method)
//     {

//       return IntPtr.Zero;
//     }

//     static MethodBuilder CreateMethodBuilder()
//     {
//       if (_typeBuilder == null)
//       {
//         var assemblyName = new AssemblyName();
//         var assembly =
//       }

//       var method = _typeBuilder.DefineMethod(
//         Guid.NewGuid().ToString(),
//         MethodAttributes.Static,
//         CallingConventions.Standard,
//         null,
//         new Type[0]
//       );

//       return method;
//     }

//     [ThreadStatic]
//     static TypeBuilder _typeBuilder;
//   }
// }