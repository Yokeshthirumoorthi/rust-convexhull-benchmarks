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
use rand_gen::rand::random;

use std::f64;
// use plots::*;

///Generates n number of random points based on the
/// shape constraint and returns them as collection of
/// Point2D
pub fn generate(number_of_points: u32, between: Range<f64>, shape: Shape) -> Vec<Point2D> {
    let mut range = rand::thread_rng();
    let mut output: Vec<Point2D> = Vec::new();

    // for _ in 0..number_of_points {
    //     let a = between.ind_sample(&mut range);
    //     let b = between.ind_sample(&mut range);
    //     output.push(Point2D::new(a, b))
    // }
    // for _ in 0..number_of_points {
    //     let t = 2.0 * (f64::consts::PI) * random::<f64>();
    //     let u = random::<f64>()+random::<f64>();
    //     let r = if u>1.0  { 2.0 - u } else { u };
        
    //     let a = r*(t.cos());
    //     let b = r*(t.sin());
    //     output.push(Point2D::new(a, b));
    // }
    let center = (0., 0.);
    let r = 1.0;
    let mut f: f64 = 0.0;
    for _ in 0..36 {
        let a = r*(f.cos());
        let b = r*(f.sin());
        f += 2.0 * f64::consts::PI / 36.0;
        output.push(Point2D::new(a, b));
    }

    // println!("{:?}", output);
    // draw_plot(&output);
    output
}