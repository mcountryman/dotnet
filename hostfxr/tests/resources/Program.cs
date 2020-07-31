using System;
using System.Runtime.InteropServices;

namespace HostFxrTest {
    public class HostFxr {
        public delegate int TestDelegate(string p1, int p2, long p3);

        public int Test([MarshalAs(UnmanagedType.LPStr)] string p1, int p2, long p3) {
            Console.WriteLine($"p1: {p1}");
            Console.WriteLine($"p2: {p2}");
            Console.WriteLine($"p3: {p3}");

            return 420;
        }
    }
}