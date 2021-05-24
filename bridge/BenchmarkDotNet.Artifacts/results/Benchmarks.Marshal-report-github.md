``` ini

BenchmarkDotNet=v0.13.0, OS=macOS Big Sur 11.2 (20D5042d) [Darwin 20.3.0]
Intel Core i9-9880H CPU 2.30GHz, 1 CPU, 16 logical and 8 physical cores
.NET SDK=5.0.101
  [Host]     : .NET 5.0.1 (5.0.120.57516), X64 RyuJIT
  DefaultJob : .NET 5.0.1 (5.0.120.57516), X64 RyuJIT


```
| Method |      Mean |     Error |    StdDev |    Median |
|------- |----------:|----------:|----------:|----------:|
| Sha256 | 0.1071 ns | 0.0285 ns | 0.0751 ns | 0.1100 ns |
|    Md5 | 0.0217 ns | 0.0142 ns | 0.0390 ns | 0.0000 ns |
