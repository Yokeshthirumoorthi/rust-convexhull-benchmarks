extern crate time;
use self::time::{Duration, PreciseTime};

use inputset_gen::*;
use points::*;
use convexhull::*;

///Types of algorithms handled in this programm
enum Algorithm {
    Graham,
    Jarvis,
    Chan,
}

///Types of shapes used for input sampling
#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Triangle,
    Rectangle,
    Circle,
}

impl Shape {
    pub fn num_of_vertices(self) -> u64 {
        match self {
            Shape::Triangle => 3,
            Shape::Rectangle => 4,
            // we have chosen to have 18 vertex points for a triangle
            Shape::Circle => 18,
        }
    }
}

///Sizes of input sampling
#[derive(Debug, Copy, Clone)]
pub enum Number {
    Hundred,
    Thousand,
    TenThousand,
    HundredThousand,
    Million,
    TenMillion,
}

impl Number {
    pub fn val(self) -> u64 {
        match self {
            Number::Hundred => 100,
            Number::Thousand => 1_000,
            Number::TenThousand => 10_000,
            Number::HundredThousand => 100_000,
            Number::Million => 1_000_000,
            Number::TenMillion => 10_000_000,
        }
    }
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
        let runtime_nanos = duration
            .num_nanoseconds()
            .expect("Benchmark iter took greater than 2^63 nanoseconds")
            as f64;
        let runtime_secs = runtime_nanos / 1_000_000_000.0;
        let runtime_milli_secs = runtime_secs * 1_000.0;

        Time {
            seconds: runtime_secs,
            milli_seconds: runtime_milli_secs,
            nano_seconds: runtime_nanos,
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

/// Benchmarks all the 3 algorithms for same input
/// The output is printed to the console
pub fn benchmark_algorithms(shape: Shape, sample_size: Number) {
    let mut input_set: Vec<Point2D> = get_input_set(sample_size.val(), shape.num_of_vertices());

    let time_graham = execution_time(Algorithm::Graham, &mut input_set);
    let time_jarvis = execution_time(Algorithm::Jarvis, &mut input_set);
    let time_chan = execution_time(Algorithm::Chan, &mut input_set);

    // The output is printed to the console
    println!("----------------------------------------");
    println!("Shape: {:?}, Size: {:?}", shape, sample_size);
    println!("graham_scan: {:?} ms", time_graham.milli_seconds());
    println!("jarvis_march: {:?} ms", time_jarvis.milli_seconds());
    println!("chans_algorithm: {:?} ms", time_chan.milli_seconds());
    // println!("graham_scan: {:?} s", time_graham.seconds());
    // println!("jarvis_march: {:?} s", time_jarvis.seconds());
    // println!("chans_algorithm: {:?} s", time_chan.seconds());
    println!("----------------------------------------")
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
