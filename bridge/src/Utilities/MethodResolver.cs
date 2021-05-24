using System;
using System.Reflection;
using System.Collections.Generic;
using System.Linq;

namespace Dotnet.Bridge.Utilities {
  class MethodResolver {
    public static MethodInfo GetMethod(string path, BridgeObjectType retType, BridgeObjectType[] types) {
      foreach (var method in GetMethodsFromPath(path)) {
        if (IsMatchingMethod(method, retType, types)) {
          return method;
        }
      }

      throw new MissingMethodException("Method for supplied args couldn't be found");
    }

    static IEnumerable<MethodInfo> GetMethodsFromPath(string path) {
      var typeNameIndex = path.LastIndexOf(".");
      var typeName = path.Substring(0, typeNameIndex - 1);
      var type = Type.GetType(typeName);
      if (type == null)
        throw new TypeAccessException($"Type '{typeName}' not found");

      var methodName = path.Substring(typeNameIndex);
      var methods = type.GetMethods();
      if (methods.Length == 0)
        throw new MissingMethodException($"Type '{typeName}' doesn't contain methods");

      return methods.Where(x => x.Name == methodName);
    }

    static bool IsMatchingMethod(MethodInfo method, BridgeObjectType retType, BridgeObjectType[] types) {
      if (!method.ReturnType.IsAssignableFrom(retType.AsType()))
        return false;

      var parameters = method.GetParameters();
      if (parameters.Length != types.Length)
        return false;

      for (var i = 0; i < types.Length; i++) {
        var type = types[i].AsType();
        var parameterType = parameters[i].ParameterType;

        if (!type.IsAssignableTo(parameterType)) {
          return false;
        }
      }

      return true;
    }
  }
}