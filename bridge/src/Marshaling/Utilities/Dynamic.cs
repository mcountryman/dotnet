using System;
using System.Reflection;
using System.Reflection.Emit;

namespace Dotnet.Bridge.Marshaling.Utilities
{
  public class Dynamic
  {
    public static TypeBuilder CreateStaticType()
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
          var assemblyName = new AssemblyName("Dotnet.Bridge.Marshaling");
          var assemblyBuilder = AssemblyBuilder.DefineDynamicAssembly(assemblyName, AssemblyBuilderAccess.Run);
          var moduleBuilder = assemblyBuilder?.DefineDynamicModule("Dotnet.Bridge.Marshaling");
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