// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Collection of functions that operates and
//! modifies the input set to conveniently
//! determine the hull points
use points::*;

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
    Other(u64),
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
            Number::Other(x) => x,
        }
    }

    pub fn times(self, x: u64) -> Number {
        Number::Other(self.val() * x)
    }
}

///Converts any raw point tuple to Point2D type
/// and pushes it to the inputset
pub fn push(inputset: &mut Vec<Point2D>, point: (f64, f64)) -> &Vec<Point2D> {
    let point = Point2D::new(point.0, point.1);
    inputset.push(point);
    inputset
}

///Finds the point in input set with least y-coordinate or the
///left most in case of a match
/// Sets the pivot point of the inputset
/// as the first element of the input set.
pub fn set_pivot(input_set: &mut Vec<Point2D>) -> &Vec<Point2D> {
    //panic if there are no elements in the input_set
    assert!(input_set.len() > 0);
    //set the vertex point to be the
    //first element of the inputset
    for i in 0..input_set.len() {
        if input_set[i].pick_left(&input_set[0]) == &input_set[i] {
            &input_set.swap(0, i);
        }
    }
    input_set
}

///Sorts the  elements of input set by polar
///angle in counter clockwise order around pivot point.
///(if more than one point has the same angle, remove all
///but the one that is farthest from pivot point)
pub fn sort_polar_angle_ccw(input_set: &Vec<Point2D>) -> Vec<Point2D> {
    let mut fat_pt_vec: Vec<Fatpoint2D> = Vec::new();
    //convert all the point2D as FatPoints
    if let Some((first, elements)) = input_set.split_first() {
        fat_pt_vec = elements
            .iter()
            .map(|point| Fatpoint2D::new(point, &first))
            .collect();
    };
    //sort the fatpoint vec
    fat_pt_vec.sort_by(|a, b| b.partial_cmp_distance(a).unwrap());
    fat_pt_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    fat_pt_vec.dedup_by(|a, b| a.eq_polar_angle(b));
    //convert back to point2d
    let mut output = Vec::new();
    if let Some(first) = input_set.get(0) {
        output.push(Fatpoint2D::new(first, &first).to_point())
    }
    for fat_point in fat_pt_vec.iter() {
        output.push(fat_point.to_point());
    }
    output
}

pub fn jarvis_binary_search(
    next_to_top: &Point2D,
    top: &Point2D,
    sub_hull_set: &Vec<Point2D>,
) -> Point2D {
    let mut max_angled_point = sub_hull_set[0];
    let mut max_angle = 0.0;
    // println!("next_to_top {:?}",next_to_top);
    // println!("top {:?}",top);
    // println!("sub_hull_set {:?}",sub_hull_set);
    for point in sub_hull_set {
        let orientation = find_angle(&next_to_top, &top, &point);
        // if next_to_top == &Point2D::new(2.,0.) {
        //     println!("next_to_top {:?}",next_to_top);
        //     println!("top {:?}",top);
        // println!("{:?}: {}",*point, orientation);
        // }

        if max_angle < orientation && *point != *top {
            max_angle = orientation;
            max_angled_point = *point
        }
    }
    max_angled_point
}

/// Generates set of points with predetermined number
/// of hull vertices and in the given shape
use std::f64;
// use plots::*;
extern crate rand;
use self::rand::prelude::*;
/// Generate the input set for convex hull
///
/// Idea is derived from this link
/// https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly/5838055#5838055
///
/// # Panics
/// Panics if the total points is less than the number of vertices
// pub fn generate(total_points: u64, number_of_vertex: u64) -> Vec<Point2D> {
pub fn generate(shape: Shape, sample_size: Number) -> Vec<Point2D> {
    let total_points = sample_size.val();
    let number_of_vertex = shape.num_of_vertices();
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