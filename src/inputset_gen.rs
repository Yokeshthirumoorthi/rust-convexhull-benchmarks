//! Generates set of points with predetermined number
//! of hull vertices and in the given shape
use points::Point2D;
use std::f64;
// use plots::*;
extern crate rand;
use inputset_gen::rand::prelude::*;
/// Generate the input set for convex hull
/// 
/// Idea is derived from this link
/// https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly/5838055#5838055
/// 
/// # Panics
/// Panics if the total points is less than the number of vertices
pub fn get_input_set(total_points: u64, number_of_vertex: u64) -> Vec<Point2D>{
    assert!(total_points >= number_of_vertex);
    let mut output: Vec<Point2D> = Vec::new();
    let radius = 2.0;
    let number_of_fill_points = total_points - number_of_vertex;
    let mut theta: f64 = 0.0;

    for _ in 0..number_of_vertex {
        // x = r cos(theta)
        let x_co_ordinate = radius * (theta.cos());
        // y = r sin(theta)
        let y_co_ordinate = radius * (theta.sin());
        // add the co-ordinates as Point2D to the output set
        output.push(Point2D::new(x_co_ordinate, y_co_ordinate));
        // increment theta for next vertex
        theta += 2.0 * f64::consts::PI / number_of_vertex as f64;
    }
    
    let mut rng = thread_rng();
    for _ in 0..number_of_fill_points {
        // calculate a random theta for each point
        theta = 2.0 * f64::consts::PI * random::<f64>();
        // random radius in range (0, 1)
        let radius: f64 = rng.gen();
        // x = r cos(theta)
        let x_co_ordinate = radius * (theta.cos());
        // y = r sin(theta)
        let y_co_ordinate = radius * (theta.sin());
        // add the co-ordinates as Point2D to the output set
        output.push(Point2D::new(x_co_ordinate, y_co_ordinate));
    }
    // draw_plot(&output);
    output
} 