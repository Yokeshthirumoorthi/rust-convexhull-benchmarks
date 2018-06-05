// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate time;
use self::time::{Duration, PreciseTime};

use points::*;
use convexhull::*;
use inputset::*;

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

/// One another convenient function to print the results
/// The output is printed in console
pub fn benchmark_algorithm(algorithm: Algorithm, shape: Shape) {
    use self::Number::*;
    let sample_sizes: Vec<Number> = vec![
        Hundred,
        Thousand,
        TenThousand,
        HundredThousand,
        HundredThousand.times(2),
        HundredThousand.times(5),
        HundredThousand.times(7),
        Million,
        Million.times(2),
        Million.times(5),
        Million.times(7),
        TenMillion,
    ];

    let mut output: Vec<(u64, f64)> = Vec::new();
    for sample_size in sample_sizes {
        let mut input_set: Vec<Point2D> = generate(shape, sample_size);
        let result = (
            sample_size.val(),
            execution_time(algorithm, &mut input_set).milli_seconds(),
        );
        output.push(result);
    }
    println!("{:?}_{:?} : {:?}", algorithm, shape, output);
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
