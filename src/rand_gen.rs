//! Creates random points within a given shape and its boundaries 

use points::Point2D;

extern crate rand;
use rand_gen::rand::distributions::{Range};
// use rand_gen::rand::random;

use std::f64;
// use plots::*;

/// Shapes within which we generate the randon points.
/// 
/// # Examples
/// 
/// ```
/// use rustalgo::points::Point2D;
/// let circle = rustalgo::rand_gen::Shape::Circle {radius: 1.0, center: Point2D::new(0.0,0.0)};
/// assert_eq!(rustalgo::rand_gen::Shape::Circle.radius, 1.0);
/// ```
/// 
pub enum Shape {
    Circle {radius: f64, center: Point2D},
    Rectangle {height: f64, width: f64, center: Point2D},
    Triangle {height: f64, width: f64, vertex: Point2D}
}

/// Implementation methods for shapes
/// 
/// # Examples
/// 
/// ```
/// use rustalgo::points::Point2D;
/// let origin = Point2D::new(0.0, 0.0);
/// let circle = Shape::new_circle(1.0, origin);
/// assert_eq!(Shape::Circle {radius: 1.0, center: origin }, circle);
/// let rectangle = Shape::new_rectangle(2.0, 2.0, origin);
/// assert_eq!(Shape::Rectangle {height: 2.0, width: 2.0, center: origin }, circle);
/// let triangle = Shape::new_triangle(1.0, 2.0, origin);
/// assert_eq!(Shape::Triangle {height: 1.0, width: 2.0, center: origin }, circle);
/// ```
/// 
impl Shape {
    pub fn new_triangle(height: f64, width: f64, vertex: Point2D) -> Shape {
        Shape::Triangle {height, width, vertex}
    }
    pub fn new_rectangle(height: f64, width: f64, center: Point2D) -> Shape {
        Shape::Rectangle {height, width, center}
    }
    pub fn new_circle(radius: f64, center: Point2D) -> Shape {
        Shape::Circle {radius, center}
    }
}

///Generates n number of random points based on the
/// shape constraint and returns them as collection of
/// Point2D
pub fn generate(_number_of_points: u32, _between: Range<f64>, _shape: Shape) -> Vec<Point2D> {
    // let mut range = rand::thread_rng();
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
    // let center = (0., 0.);
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