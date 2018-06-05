# RUSTALGO

Copyright (c) 2018 Yokesh Thirumoorthi

A Rust library for benchmarking the following convex hull algorithms.

1. Graham Scan
2. Jarvis March or Gift Wrapping Algorithm
3. Chan's Algorithm

## Executing and Testing this programm
To install Rust, run the following in your terminal

```bash
curl https://sh.rustup.rs -sSf | sh
```

or follow the installation instruction from https://www.rust-lang.org/en-US/install.html.

Then run the following script

```bash
git clone https://github.com/Yokeshthirumoorthi/rustalgo.git
cd rustalgo
cargo build
cargo run --release
```

To run the test use

```bash
cargo test
```

## Usage

To use this library as a crate in your code, add this to your `Cargo.toml`:

```toml
[dependencies]
rustalgo = "0.1"
```

```rust
extern crate rustalgo;
//Detailed information about the api are available in the rust doc
//generated using cargo doc --open
use rustalgo::benchmark::benchmark_algorithm;
use rustalgo::benchmark::Shape::*;
use rustalgo::benchmark::Algorithm::*;

fn main() {
    println!("Benchmark Results");
    //Triangle
    benchmark_algorithm(Graham, Triangle);
    benchmark_algorithm(Jarvis, Triangle);
    benchmark_algorithm(Chan, Triangle);
    //Rectangle
    benchmark_algorithm(Graham, Rectangle);
    benchmark_algorithm(Jarvis, Rectangle);
    benchmark_algorithm(Chan, Rectangle);
    //Circle
    benchmark_algorithm(Graham, Circle);
    benchmark_algorithm(Jarvis, Circle);
    benchmark_algorithm(Chan, Circle);
}
```
## Author
1. Yokesh Thirumoorthi - initial author - yokeshthirumoorthi@gmail.com

## License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.