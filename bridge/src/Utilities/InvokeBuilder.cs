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
      var signature = GetDelegate(wrapper);
      var del = Activator.CreateInstance(signature, new[] {wrapper});

      return Marshal.GetFunctionPointerForDelegate(del!);
    }

    static MethodInfo GetWrapper(
      MethodInfo target,
      BridgeObjectType retType,
      BridgeObjectType[] types
    ) {
      var methodName = new Guid().ToString();
      var method = new DynamicMethod(
        methodName,
        MethodAttributes.Public | MethodAttributes.Static | MethodAttributes.UnmanagedExport,
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

    static Type GetDelegate(MethodInfo methodInfo) {
      var module = GetModuleBuilder();

      var signatureName = new Guid().ToString();
      var signature = module.DefineType(
        signatureName,
        TypeAttributes.Sealed | TypeAttributes.Public,
        typeof(MulticastDelegate)
      );

      signature.DefineConstructor(
        MethodAttributes.RTSpecialName | MethodAttributes.HideBySig | MethodAttributes.Public,
        CallingConventions.Standard,
        new[] {
          typeof(object),
          typeof(IntPtr)
        }
      );

      var parameters = methodInfo.GetParameters();
      var invoke = signature.DefineMethod(
        "Invoke",
        MethodAttributes.HideBySig | MethodAttributes.Virtual | MethodAttributes.Public,
        methodInfo.ReturnType,
        parameters.Select(p => p.ParameterType).ToArray()
      );

      invoke.SetImplementationFlags(MethodImplAttributes.CodeTypeMask);

      for (var i = 0; i < parameters.Length; i++) {
        var parameter = parameters[i];
        invoke.DefineParameter(i + 1, ParameterAttributes.None, parameter.Name);
      }

      signature.SetCustomAttribute(GetDelegateAttribute());

      return signature.CreateType()!;
    }

    static CustomAttributeBuilder GetDelegateAttribute() {
      var ctor = typeof(UnmanagedFunctionPointerAttribute)
        .GetConstructor(new[] {typeof(CallingConvention)});
      return new CustomAttributeBuilder(ctor, new object[] {CallingConvention.Cdecl});
    }

    static ModuleBuilder GetModuleBuilder() {
      if (_module != null)
        return _module;

      var assemblyName = new AssemblyName(new Guid().ToString());
      var assembly = AssemblyBuilder.DefineDynamicAssembly(assemblyName, AssemblyBuilderAccess.Run);
      var module = assembly.DefineDynamicModule("InvokeBuilderModule");

      _module = module;
      return module;
    }

    [ThreadStatic] static ModuleBuilder? _module;
  }
}