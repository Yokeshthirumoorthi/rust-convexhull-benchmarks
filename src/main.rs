extern crate rustalgo;
use rustalgo::benchmark::benchmark_algorithms;
use rustalgo::benchmark::Shape::*;
use rustalgo::benchmark::Number::*;

fn main() {
    println!("Benchmark For Smaple Data");
    benchmark_algorithms(Triangle, Hundred);
    benchmark_algorithms(Triangle, Thousand);
    benchmark_algorithms(Triangle, TenThousand);
    // benchmark_algorithms(Triangle, HundredThousand);
    // benchmark_algorithms(Triangle, Million);
    // benchmark_algorithms(Triangle, TenMillion);
    benchmark_algorithms(Rectangle, Hundred);
    benchmark_algorithms(Rectangle, Thousand);
    benchmark_algorithms(Rectangle, TenThousand);
    benchmark_algorithms(Circle, Hundred);
    benchmark_algorithms(Circle, Thousand);
    benchmark_algorithms(Circle, TenThousand);
}
