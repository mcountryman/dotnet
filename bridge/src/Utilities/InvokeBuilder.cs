using System;
using System.Linq;
using System.Reflection;
using System.Reflection.Emit;
using System.Collections.Generic;
using System.Linq.Expressions;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge.Utilities {
  public static class InvokeBuilder {
    public static IntPtr Prepare(
      string path,
      BridgeObjectType retType,
      BridgeObjectType[] types
    ) {
      var target = MethodResolver.GetMethod(path, retType, types);
      var wrapper = GetWrapper(target, retType, types);

      return Marshal.GetFunctionPointerForDelegate(wrapper.CreateDelegate(typeof(Action)));
    }

    static MethodInfo GetWrapper(
      MethodInfo target,
      BridgeObjectType retType,
      BridgeObjectType[] types
    ) {
      var methodName = new Guid().ToString();
      var method = new DynamicMethod(
        methodName,
        MethodAttributes.Public | MethodAttributes.Static,
        CallingConventions.Standard,
        retType.AsType(),
        types.Select(x => x.AsType()).ToArray(),
        typeof(InvokeBuilder).Module,
        false
      );

      var il = method.GetILGenerator();

      for (var i = 0; i < types.Length; i++) {
        il.Emit(OpCodes.Ldarg, i);
      }

      il.EmitCall(OpCodes.Call, target, null);
      il.Emit(OpCodes.Ret);

      return method;
    }

    static Type GetDelegate(
      BridgeObjectType retType,
      BridgeObjectType[] types
    ) {
      var delTypes = types.Concat(new[] { retType }).Select(x => x.AsType()).ToArray();
      var del = Expression.GetDelegateType(delTypes);

      return del;
    }

    static ModuleBuilder GetModuleBuilder() {
      var assemblyName = new AssemblyName(Guid.NewGuid().ToString("N"));
      var assembly = AssemblyBuilder.DefineDynamicAssembly(
        assemblyName,
        AssemblyBuilderAccess.RunAndCollect
      );

      var moduleName = Guid.NewGuid().ToString("N");
      var module = assembly.DefineDynamicModule(moduleName);

      return module;
    }

    [ThreadStatic]
    static ModuleBuilder? _moduleBuilder;
  }
}