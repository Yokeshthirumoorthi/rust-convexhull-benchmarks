use std::fs::File;
use std::io::prelude::*;

extern crate time;
use time::{PreciseTime, Duration};

extern crate rustalgo;
use rustalgo::inputset_gen::*;
use rustalgo::sample_data::*;
use rustalgo::points::*;
use rustalgo::convexhull::*;

fn main() {
    generate_sample_to_file(100, 3);
    let mut triangle_10: Vec<Point2D> = triangle_10().iter().map(|p| Point2D::new(p.0,p.1)).collect();
    println!("Benchmark For Smaple Data");
    println!("Shape::Triangle, Input size: {}", 10);
    benchmark_convex_hull_algorithms(&mut triangle_10);
    println!("----------------------------------------")
}

/// Generates sample set of data and saves to a file.
/// 
/// For now, the output is saved to sample.txt and 
/// manullay moveed out of the file
/// to a rs file.
fn generate_sample_to_file (sample_size: u64, vertex_count: u64) {
    let file_name = "sample.txt";
    let mut file = File::create(file_name).unwrap();
    let sample_set = get_input_set(sample_size, vertex_count);
    let mut output = String::new();
    for point in sample_set {
        output += &format!("({},{}),", point.x, point.y);
    }
    file.write_all(output.as_bytes()).unwrap();
}

/// Benchmarks all the 3 algorithms for same input
/// The output is printed to the console
fn benchmark_convex_hull_algorithms(input_set: &mut Vec<Point2D>) {
    //graham scan algorithm
    let start = PreciseTime::now();
    graham_scan(input_set);
    let end = PreciseTime::now();
    let time = Time::new(start.to(end));
    // println!("graham_scan: {:?} s", time.seconds());
    println!("graham_scan: {:?} ms", time.milli_seconds());
    // println!("graham_scan: {:?} ns", time.nano_seconds());
    //jarvis march algorithm
    let start = PreciseTime::now();
    jarvis_march(input_set);
    let end = PreciseTime::now();
    let time = Time::new(start.to(end));
    println!("jarvis_march: {:?} ms", time.milli_seconds());
    //chans algorithm
    // let start = PreciseTime::now();
    // chans_algorithm(input_set);
    // let end = PreciseTime::now();
    // let time = Time::new(start.to(end));
    // println!("chans_algorithm: {:?} s", time.seconds());
}

/// Provides the duration in various
/// time units
#[derive(Debug, Copy, Clone)]
pub struct Time {
    seconds: f64,
    milli_seconds: f64,
    nano_seconds: f64,
}

/// implementation for accessing time duration
/// in various time units
impl Time {
    pub fn new(duration: Duration) -> Time {
        let runtime_nanos = duration.num_nanoseconds().expect("Benchmark iter took greater than 2^63 nanoseconds") as f64;
        let runtime_secs = runtime_nanos / 1_000_000_000.0;
        let runtime_milli_secs = runtime_secs * 1_000.0;

        Time {
            seconds: runtime_secs,
            milli_seconds: runtime_milli_secs,
            nano_seconds: runtime_nanos 
        }
    }

    pub fn seconds(&self) -> f64 {
        self.seconds
    }
    
    pub fn milli_seconds(&self) -> f64 {
        self.milli_seconds
    }
    
    pub fn nano_seconds(&self) -> f64 {
        self.nano_seconds
    }
}