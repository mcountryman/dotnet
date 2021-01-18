
using System;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Dotnet.Bridge.Marshaling
{
  // public enum ClrObjectType
  // {
  //   Int16,
  //   UInt16,
  //   Int32,
  //   UInt32,
  //   Int64,
  //   UInt64,
  //   Float,
  //   Double,
  //   String,
  //   Unknown,
  // }

  // [StructLayout(LayoutKind.Sequential)]
  // public struct ClrObject<T> where T : struct
  // {
  //   public ClrObjectType Type;
  //   public ushort Size;
  //   public T Value;

  //   public ClrObject(T value)
  //   {
  //     Type = TypeIdStore.GetTypeId(value);
  //     Size = DefaultSize;
  //     Value = value;
  //   }

  //   static readonly ushort DefaultSize = (ushort)Marshal.SizeOf(default(T));
  // }

  // public static class TypeIdStore
  // {
  //   public static ClrObjectType GetTypeId<T>(T value) => GetTypeId(typeof(T));
  //   public static ClrObjectType GetTypeId(Type type)
  //   {
  //     if (type == typeof(int)) return ClrObjectType.Int32;

  //     return ClrObjectType.Unknown;
  //   }
  // }

  // public static class Serializer
  // {
  //   public static int Serialize(int from) => from;
  //   public static uint Serialize(uint from) => from;
  //   public static short Serialize(short from) => from;
  //   public static ushort Serialize(ushort from) => from;
  //   public static long Serialize(long from) => from;
  //   public static float Serialize(float from) => from;
  //   public static ulong Serialize(ulong from) => from;
  //   public static double Serialize(double from) => from;
  //   public static decimal Serialize(decimal from) => from;
  //   public static IntPtr Serialize(string from) => Marshal.StringToHGlobalUni(from);
  //   public static IntPtr Serialize(MethodInfo from)
  //   {
  //     throw new NotImplementedException();
  //   }
  // }

  // public static class Deserializer
  // {
  //   public static int DeserializeInt32(int from) => from;
  //   public static uint DeserializeUInt32(uint from) => from;
  //   public static short DeserializeInt16(short from) => from;
  //   public static ushort DeserializeUInt16(ushort from) => from;
  //   public static long DeserializeUInt64(long from) => from;
  //   public static ulong DeserializeUInt64(ulong from) => from;
  //   public static float DeserializeFloat(float from) => from;
  //   public static double DeserializeDouble(double from) => from;
  //   public static decimal DeserializeDecimal(decimal from) => from;
  //   public static string DeserializeString(IntPtr from) => Marshal.PtrToStringUTF8(from)!;
  // }
}
