extern crate rustalgo;
use rustalgo::benchmark::*;

fn main() {
    println!("Benchmark For Smaple Data");
    benchmark_algorithms(Shape::Triangle, Number::Hundred);
    benchmark_algorithms(Shape::Triangle, Number::Thousand);
    benchmark_algorithms(Shape::Triangle, Number::TenThousand);
    benchmark_algorithms(Shape::Triangle, Number::HundredThousand);
    benchmark_algorithms(Shape::Triangle, Number::Million);
    benchmark_algorithms(Shape::Triangle, Number::TenMillion);
    benchmark_algorithms(Shape::Rectangle, Number::Hundred);
    benchmark_algorithms(Shape::Rectangle, Number::Thousand);
    benchmark_algorithms(Shape::Rectangle, Number::TenThousand);
    benchmark_algorithms(Shape::Circle, Number::Hundred);
    benchmark_algorithms(Shape::Circle, Number::Thousand);
    benchmark_algorithms(Shape::Circle, Number::TenThousand);
}
