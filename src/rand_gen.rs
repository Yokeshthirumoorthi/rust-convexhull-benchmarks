//! Creates random points within a given range 

///Defines possible shapes for the collections of points.
pub enum Shape {
    Circle,
    Rectangle,
    Triangle
}

use points::Point2D;

extern crate rand;
use rand_gen::rand::distributions::{IndependentSample, Range};

///Generates n number of random points based on the
/// shape constraint and returns them as collection of
/// Point2D
pub fn generate(number_of_points: u32, between: Range<f64>, shape: Shape) -> Vec<Point2D> {
    let mut range = rand::thread_rng();
    let mut output: Vec<Point2D> = Vec::new();

    for _ in 0..number_of_points {
        let a = between.ind_sample(&mut range);
        let b = between.ind_sample(&mut range);
        output.push(Point2D::new(a, b))
    }
    
    output
}