#nullable enable

using System;

public class Bridge
{
  public delegate BridgeContextHandle GetContextHandleFn();

  public static IBridgeContext GetContext() {
    lock (_contextLock) {
      if (_context == null)
        _context = null;

      return _context;
    }
  }

  public static BridgeContextHandle GetContextHandle()
  {
    return GetContext().GetHandle();
  }

  private static IBridgeContext? _context;
  private static object _contextLock = new object();
}

public class Program
{
  public static void Main(string[] args)
  {
  }
}