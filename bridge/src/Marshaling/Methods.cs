#nullable enable

using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Reflection.Emit;
using System.Runtime.InteropServices;
using System.Threading;

namespace Dotnet.Bridge.Marshaling
{
  public class DelegateMarshaller : Marshaler<Delegate, IntPtr>
  {
    public override IntPtr MarshalTo(Delegate managed)
    {
      var methodInfo = RuntimeReflectionExtensions.GetMethodInfo(managed);
      if (methodInfo == null)
      {
        throw new InvalidOperationException($"MethodInfo for delegate `{managed}` not found.");
      }

      var (returnMarshaller, returnType) = GetReturnType(methodInfo);
      var (paramMarshallers, paramTypes) = GetParamTypes(methodInfo);
      var parameters = methodInfo.GetParameters();

      var wrapperClass = CreateType();
      var wrapperName = $"Wrapped{methodInfo.Name}";
      var wrapperCtor = wrapperClass.DefineConstructor(MethodAttributes.Static | MethodAttributes.Public, CallingConventions.Any, new Type[0]);
      var wrapper = new DynamicMethod(
        wrapperName,
        MethodAttributes.Public | MethodAttributes.Static,
        CallingConventions.Standard,
        returnType,
        paramTypes.SelectMany(x => x).ToArray(),
        wrapperClass,
        false
      );

      var il = wrapper.GetILGenerator();
      var ctorIl = wrapperCtor.GetILGenerator();
      var locals = new List<LocalBuilder>();

      // Push parameter map calls and field assignments
      for (var i = 0; i < parameters.Length; i++)
      {
        var parameter = parameters[i];
        var marshaller = paramMarshallers[i];
        var marshalFromInfo = marshaller
          .GetType()
          .GetMethod(nameof(MarshalFrom), new[] { typeof(object[]) })!;

        var local = il.DeclareLocal(marshaller.ManagedType);

        for (var j = 0; j < marshaller.NativeTypes.Length; j++)
        {
          il.Emit(OpCodes.Ldarg, i + j);
        }

        il.EmitCall(OpCodes.Calli, marshalFromInfo, null);
        il.Emit(OpCodes.Stloc, i);

        locals.Add(local);
      }

      // Push parameter loads
      for (var i = 0; i < parameters.Length; i++)
      {
        var marshaller = paramMarshallers[i];

        for (var j = 0; j < marshaller.NativeTypes.Length; j++)
        {
          il.Emit(OpCodes.Ldloc, locals[i]);
        }
      }

      il.EmitCall(OpCodes.Call, methodInfo, null);

      return Marshal.GetFunctionPointerForDelegate(wrapper);
    }

    public override Delegate MarshalFrom(IntPtr managed)
    {
      // TODO: Think about this very carefully as things could go horribly wrong here..
      throw new NotImplementedException();
    }

    public (IMarshaler, Type) GetReturnType(MethodInfo methodInfo)
    {
      var returnMarshaller = MarshalContext.Resolve(methodInfo.ReturnType);
      if (returnMarshaller == null)
      {
        throw new InvalidOperationException($"Bad return type");
      }

      var returnType = returnMarshaller.NativeTypes.FirstOrDefault();
      if (returnType == null)
      {
        throw new InvalidOperationException("Bad return types");
      }

      return (returnMarshaller, returnType);
    }

    public (List<IMarshaler>, List<Type[]>) GetParamTypes(MethodInfo methodInfo)
    {
      var types = new List<Type[]>();
      var marshallers = new List<IMarshaler>();

      foreach (var parameter in methodInfo.GetParameters())
      {
        var marshaller = MarshalContext.Resolve(parameter.ParameterType);
        if (marshaller == null)
        {
          throw new InvalidOperationException("Bad parameter type");
        }

        types.Add(marshaller.NativeTypes);
        marshallers.Add(marshaller);
      }

      return (marshallers, types);
    }


    public FieldInfo GetNewMarshalerField(TypeBuilder type, IMarshaler marshaller)
    {
      return type.DefineField(new Guid().ToString(), marshaller.GetType(), FieldAttributes.Static | FieldAttributes.InitOnly);
    }

    public static TypeBuilder CreateType()
    {
      var module = GetModuleBuilder();
      var name = Guid.NewGuid().ToString();

      return module.DefineType(name, TypeAttributes.Public | TypeAttributes.Sealed);
    }

    private static ModuleBuilder GetModuleBuilder()
    {
      lock (_moduleBuilderLock)
      {
        if (_moduleBuilder == null)
        {
          var assemblyName = new AssemblyName("BridgeDelegateMarshaller");
          var assemblyBuilder = AssemblyBuilder.DefineDynamicAssembly(assemblyName, AssemblyBuilderAccess.Run);
          var moduleBuilder = assemblyBuilder?.DefineDynamicModule("BridgeDelegateMarshaller");
          if (moduleBuilder == null)
            throw new InvalidOperationException("Failed to instantiate module builder");

          _moduleBuilder = moduleBuilder;
        }

        return _moduleBuilder;
      }
    }

    private static ModuleBuilder? _moduleBuilder;
    private static readonly object _moduleBuilderLock = new object();
  }
}