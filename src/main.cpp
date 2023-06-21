#include "cpprust/src/lib.rs.h"
#include <benchmark/benchmark.h>
#include <iostream>

static void BM_rust_from_cpp(benchmark::State& state)
{
  for(auto _ : state)
  {
    rust_from_cpp("TEST");
  }
}

BENCHMARK(BM_rust_from_cpp);


int main(int argc, char** argv)
{
    init_rust_logger();
    ::benchmark::Initialize(&argc, argv);
    ::benchmark::RunSpecifiedBenchmarks();
    
    return 0;
}