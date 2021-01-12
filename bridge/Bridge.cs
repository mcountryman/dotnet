using System;


public class Bridge {
  public delegate void InitializeFn();

  public void Initialize() {
    Console.WriteLine("Initialize");
  }
}

public class Program {
  public static void Main(string[] args) {
    var argss = String.Join(", ", args);

    Console.WriteLine($"Dotnet.Bridge.Initialize: [{argss}]");
  }
}