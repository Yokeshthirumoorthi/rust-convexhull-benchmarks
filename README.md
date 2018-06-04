# RUSTALGO

A Rust library for benchmarking the following convex hull algorithms.

1. Graham Scan
2. Jarvis March or Gift Wrapping Algorithm
3. Chan's Algorithm

The input set is generated using rand crate for some 
predetermined hull shape. For example, the sample input set
generated for a triangulr hull has 3 hull points and the rest of the points fill within these vertices.

The permissible shapes are Triangle, Rectangle 
and Circle. And the permissible input sizes
are Hundred, Thousand, TenThousand, HundredThousand, Million, TenMillion.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustalgo = "0.1"
```

```rust
extern crate rustalgo;
use rustalgo::benchmark::benchmark_algorithm;
use rustalgo::benchmark::Shape::*;
use rustalgo::benchmark::Algorithm::*;

fn main() {
    println!("Benchmark Results");
    benchmark_algorithm(Graham, Triangle);
    benchmark_algorithm(Jarvis, Triangle);
    benchmark_algorithm(Chan, Triangle);
    benchmark_algorithm(Graham, Rectangle);
    benchmark_algorithm(Jarvis, Rectangle);
    benchmark_algorithm(Chan, Rectangle);
    benchmark_algorithm(Graham, Circle);
    benchmark_algorithm(Jarvis, Circle);
    benchmark_algorithm(Chan, Circle);
}
```
## Author
1. Yokesh Thirumoorthi - yokeshthirumoorthi@gmail.com

## License

Rustalgo is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.