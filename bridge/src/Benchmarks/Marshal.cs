using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using BenchmarkDotNet.Attributes;

namespace Benchmarks {
  public class Marshaller {
    byte[] Heap;
    (short, int, long, byte[], string) Register;

    public Marshaller() {
      Register = (69, 420, 1337, Encoding.UTF8.GetBytes("test"), "test");
      Heap = BitConverter.GetBytes(Register.Item1)
        .Concat(BitConverter.GetBytes(Register.Item2))
        .Concat(BitConverter.GetBytes(Register.Item3))
        .Concat(BitConverter.GetBytes(Register.Item4.Length))
        .Concat(Register.Item4)
        .ToArray();
    }

    [Benchmark]
    public void HeapPointer() {
      unsafe {
        fixed (byte* heap = Heap) {
          var (i16, i32, i64, str) = MarshalImpl.HeapPointer(heap, Heap.Length);

          Trace.Assert(i16 == Register.Item1);
          Trace.Assert(i32 == Register.Item2);
          Trace.Assert(i64 == Register.Item3);
          Trace.Assert(str == Register.Item5);
        }
      }

    }

    [Benchmark]
    public void HeapRefByte() {
      var (i16, i32, i64, str) = MarshalImpl.HeapRefByte(ref Heap);

      Trace.Assert(i16 == Register.Item1);
      Trace.Assert(i32 == Register.Item2);
      Trace.Assert(i64 == Register.Item3);
      Trace.Assert(str == Register.Item5);
    }

    [Benchmark]
    public void Registers() {
      var (i16, i32, i64, str) = MarshalImpl.Register(Register.Item1, Register.Item2, Register.Item3, ref Register.Item4);

      Trace.Assert(i16 == Register.Item1);
      Trace.Assert(i32 == Register.Item2);
      Trace.Assert(i64 == Register.Item3);
      Trace.Assert(str == Register.Item5);
    }
  }

  unsafe class MarshalImpl {
    [MethodImpl(MethodImplOptions.NoInlining)]
    public static (short, int, long, string) HeapPointer(byte* pointer, int size) {
      var i16 = *(short*)(pointer); pointer += 2;
      var i32 = *(int*)(pointer); pointer += 4;
      var i64 = *(long*)(pointer); pointer += 8;
      var len = *(int*)(pointer); pointer += 4;
      var ptr = new IntPtr(pointer);
      var str = Marshal.PtrToStringUTF8(ptr, len);

      return (i16, i32, i64, str);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    public static (short, int, long, string) HeapRefByte(ref byte[] buf) {
      var i = 1024;

      var i16 = BitConverter.ToInt16(buf[i..]); i += 2;
      var i32 = BitConverter.ToInt32(buf[i..]); i += 4;
      var i64 = BitConverter.ToInt64(buf[i..]); i += 8;
      var len = BitConverter.ToInt32(buf[i..]); i += 4;
      var str = Encoding.UTF8.GetString(buf[i..(i + len)]);

      return (i16, i32, i64, str);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    public static (short, int, long, string) Register(short i16, int i32, long i64, ref byte[] str) {
      return (i16, i32, i64, Encoding.UTF8.GetString(str));
    }
  }
}