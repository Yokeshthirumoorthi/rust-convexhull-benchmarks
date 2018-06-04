extern crate rustalgo;
use rustalgo::benchmark::*;
use rustalgo::benchmark::Shape::*;
// use rustalgo::benchmark::Number::*;
use rustalgo::benchmark::Algorithm::*;

fn main() {
    println!("Benchmark Results");
    // benchmark_algorithms(Triangle, Hundred);
    // benchmark_algorithms(Triangle, Thousand);
    // benchmark_algorithms(Triangle, TenThousand);
    // // benchmark_algorithms(Triangle, HundredThousand);
    // // benchmark_algorithms(Triangle, Million);
    // // benchmark_algorithms(Triangle, TenMillion);
    // benchmark_algorithms(Rectangle, Hundred);
    // benchmark_algorithms(Rectangle, Thousand);
    // benchmark_algorithms(Rectangle, TenThousand);
    // benchmark_algorithms(Circle, Hundred);
    // benchmark_algorithms(Circle, Thousand);
    // benchmark_algorithms(Circle, TenThousand);
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
