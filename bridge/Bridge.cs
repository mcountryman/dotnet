using System;


public class Bridge
{
  public delegate void InitializeFn();

  public static void Initialize()
  {
    Console.WriteLine("Initialize");
  }
}

public class Program
{
  public static void Main(string[] args)
  {
    var type = typeof(Bridge.InitializeFn);

    Console.WriteLine(type.AssemblyQualifiedName);
  }
}