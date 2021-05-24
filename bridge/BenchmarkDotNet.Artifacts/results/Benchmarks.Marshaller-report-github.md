``` ini

BenchmarkDotNet=v0.13.0, OS=macOS Big Sur 11.2 (20D5042d) [Darwin 20.3.0]
Intel Core i9-9880H CPU 2.30GHz, 1 CPU, 16 logical and 8 physical cores
.NET SDK=5.0.101
  [Host]     : .NET 5.0.1 (5.0.120.57516), X64 RyuJIT
  DefaultJob : .NET 5.0.1 (5.0.120.57516), X64 RyuJIT


```
|      Method |     Mean |    Error |   StdDev |   Median |
|------------ |---------:|---------:|---------:|---------:|
| HeapPointer | 37.90 ns | 1.060 ns | 3.124 ns | 36.97 ns |
| HeapRefByte |       NA |       NA |       NA |       NA |
|   Registers | 31.25 ns | 1.033 ns | 2.981 ns | 30.78 ns |

Benchmarks with issues:
  Marshaller.HeapRefByte: DefaultJob
