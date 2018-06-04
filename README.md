# RUSTALGO

A Rust library for benchmarking the following convex hull algorithms.

1. Graham Scan
2. Jarvis March or Gift Wrapping Algorithm
3. Chan's Algorithm

The input set is generated using rand crate for some 
predetermined hull shape. For example, the sample input set
generated for a triangulr hull has 3 hull points and the rest of the points fill within the vertices.

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
use rustalgo::benchmark::*;

fn main() {
    println!("Benchmark For Smaple Data");
    benchmark_algorithms(Shape::Triangle, Number::Hundred);
    benchmark_algorithms(Shape::Triangle, Number::Thousand);
    benchmark_algorithms(Shape::Triangle, Number::TenThousand);
    benchmark_algorithms(Shape::Rectangle, Number::Hundred);
    benchmark_algorithms(Shape::Rectangle, Number::Thousand);
    benchmark_algorithms(Shape::Rectangle, Number::TenThousand);
    benchmark_algorithms(Shape::Circle, Number::Hundred);
    benchmark_algorithms(Shape::Circle, Number::Thousand);
    benchmark_algorithms(Shape::Circle, Number::TenThousand);
}
```
## Author
1. Yokesh Thirumoorthi - yokeshthirumoorthi@gmail.com

## License

Rustalgo is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.