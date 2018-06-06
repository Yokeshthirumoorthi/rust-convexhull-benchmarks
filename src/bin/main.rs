// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rustalgo;
use rustalgo::benchmark::benchmark_algorithm;
use rustalgo::inputset::Shape::*;
use rustalgo::convexhull::Algorithm::*;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let mut presort = false;
    if args.len() > 1 {
        match args[1].as_str() {
            "presort" => {
                presort = true
            },
            _ => println!("You can use argument 'presort' to benchmark presorted input"),
        }
    }

    println!("Benchmark Results");
    benchmark_algorithm(Graham, Triangle, presort);
    benchmark_algorithm(Jarvis, Triangle, presort);
    benchmark_algorithm(Chan, Triangle, presort);
    benchmark_algorithm(Graham, Rectangle, presort);
    benchmark_algorithm(Jarvis, Rectangle, presort);
    benchmark_algorithm(Chan, Rectangle, presort);
    benchmark_algorithm(Graham, Circle, presort);
    benchmark_algorithm(Jarvis, Circle, presort);
    benchmark_algorithm(Chan, Circle, presort);
}
