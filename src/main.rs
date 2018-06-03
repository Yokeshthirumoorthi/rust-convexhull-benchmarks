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
    // generate_sample_to_file(100, Shape::Triangle);
    let mut triangle_100: Vec<Point2D> = triangle_100().iter().map(|p| Point2D::new(p.0 as f64,p.1 as f64)).collect();
    benchmark_convex_hull_algorithms(&mut triangle_100);
}

//Types of shapes used for input sampling
pub enum Shape {
    Triangle,
    Rectangle,
    // we have chosen to have 18 vertex points for a triangle
    Circle,
}

impl Shape {
    pub fn num_of_vertices(self) -> u64 {
        match self {
            Shape::Triangle => 3,
            Shape::Rectangle => 4,
            Shape::Circle => 18,
        }
    }
}

/// Generates sample set of data and saves to a file.
/// 
/// For now, the output is saved to sample.txt and 
/// manullay moveed out of the file
/// to a rs file.
pub fn generate_sample_to_file (sample_size: u64, shape: Shape) {
    let file_name = "sample.txt";
    let mut file = File::create(file_name).unwrap();
    let sample_set = get_input_set(sample_size, shape.num_of_vertices());
    let mut output = String::new();
    for point in sample_set {
        output += &format!("({},{}),", point.x, point.y);
    }
    //remove the ',' value in the end of string
    output.pop();
    // println!("{}", output);
    file.write_all(output.as_bytes()).unwrap();
}

/// Benchmarks all the 3 algorithms for same input
/// The output is printed to the console
fn benchmark_convex_hull_algorithms(input_set: &mut Vec<Point2D>) {
    println!("Benchmark For Smaple Data");
    println!("Shape::Triangle, Input size: {}", input_set.len());
    println!("graham_scan: {:?} ms", execution_time(Algorithm::Graham, input_set).milli_seconds());
    println!("jarvis_march: {:?} ms", execution_time(Algorithm::Jarvis, input_set).milli_seconds());
    println!("chans_algorithm: {:?} ms", execution_time(Algorithm::Chan, input_set).milli_seconds());
    println!("----------------------------------------")
}

///Types of algorithms handled in this programm
enum Algorithm {
    Graham,
    Jarvis,
    Chan,
}

/// Executes an algorithm for given inputset of point and returns the time
fn execution_time(algorithm: Algorithm, input_set: &mut Vec<Point2D>) -> Time {
    let start = PreciseTime::now();
    match algorithm {
        Algorithm::Graham => graham_scan(input_set),
        Algorithm::Jarvis => jarvis_march(input_set),
        Algorithm::Chan => chans_algorithm(input_set),
    };
    let end = PreciseTime::now();
    Time::new(start.to(end))
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