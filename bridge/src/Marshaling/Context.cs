
using System;
using System.Collections.Generic;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge.Marshaling
{
  public struct ClrObject<T>
  {
    public uint TypeId;
    public T Value;

    public ClrObject(T value)
    {
      Value = value;
      TypeId = TypeIdStore.GetTypeId(value);
    }
  }

  public static class TypeIdStore
  {
    public static uint GetTypeId<T>(T value) => GetTypeId(typeof(T));
    public static uint GetTypeId(Type type)
    {
      return 0;
    }

    [ThreadStatic]
    private static readonly IDictionary<uint, Type> _types = new Dictionary<uint, Type>();
  }

  public static class Serializer
  {
    public static int Serialize(int from) => from;
    public static uint Serialize(uint from) => from;
    public static short Serialize(short from) => from;
    public static ushort Serialize(ushort from) => from;
    public static long Serialize(long from) => from;
    public static float Serialize(float from) => from;
    public static ulong Serialize(ulong from) => from;
    public static double Serialize(double from) => from;
    public static decimal Serialize(decimal from) => from;
    public static IntPtr Serialize(string from) => Marshal.StringToHGlobalUni(from);
    public static IntPtr Serialize(MethodInfo from)
    {
      throw new NotImplementedException();
    }
  }

  public static class Deserializer
  {
    public static int DeserializeInt32(int from) => from;
    public static uint DeserializeUInt32(uint from) => from;
    public static short DeserializeInt16(short from) => from;
    public static ushort DeserializeUInt16(ushort from) => from;
    public static long DeserializeUInt64(long from) => from;
    public static ulong DeserializeUInt64(ulong from) => from;
    public static float DeserializeFloat(float from) => from;
    public static double DeserializeDouble(double from) => from;
    public static decimal DeserializeDecimal(decimal from) => from;
    public static string DeserializeString(IntPtr from) => Marshal.PtrToStringUTF8(from)!;
  }
}
