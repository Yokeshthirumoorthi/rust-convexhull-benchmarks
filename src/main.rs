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
