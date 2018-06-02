//! Generates set of points with predetermined number
//! of hull vertices and in the given shape
use points::Point2D;
use std::f64;
/// Generate the input set for convex hull
/// 
pub fn get_input_set(total_points: u64, number_of_vertex: u64) -> Vec<Point2D>{
    let mut output: Vec<Point2D> = Vec::new();
    let radius = 1.0;
    let mut theta: f64 = 0.0;

    for _ in 0..number_of_vertex {
        //x = r cos(theta)
        let x_co_ordinate = radius * (theta.cos());
        //y = r sin(theta)
        let y_co_ordinate = radius * (theta.sin());
        //add the co-ordinates as Point2D to the output set
        output.push(Point2D::new(x_co_ordinate, y_co_ordinate));
        //increment theta for next vertex
        theta += 2.0 * f64::consts::PI / number_of_vertex;
    }

    output
} 