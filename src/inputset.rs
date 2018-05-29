//! Collection of functions that operates and
//! modifies the input set to conveniently
//! determine the hull points
use points::*;

///Converts any raw point tuple to Point2D type
/// and pushes it to the inputset
pub fn push(inputset: &mut Vec<Point2D>, point: (f64, f64)) -> &Vec<Point2D> {
    let point = Point2D::new(point.0, point.1);
    inputset.push(point);
    inputset
}

///Finds the point in input set with least y-coordinate or the
///left most in case of a match
fn pick_vertex(input_set: &Vec<Point2D>) -> &Point2D {
    //panic if there are no elements in the input_set
    assert!(input_set.len() > 0);
    //set the vertex point to be the
    //first element of the inputset
    let mut vertex_point = &input_set[0];
    for point in input_set {
        vertex_point = &point.pick_left(vertex_point);
    }
    vertex_point
}

/// Sets the pivot point of the inputset
/// as the first element of the input set.
pub fn set_pivot(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //TODO: implement the functionality
    Vec::new()
}

///Sorts the  elements of input set by polar
///angle in counter clockwise order around pivot point.
///(if more than one point has the same angle, remove all
///but the one that is farthest from pivot point)
pub fn sort_polar_angle_ccw(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //TODO: implement the functionality
    Vec::new()
}

#[test]
fn test_pick_vertex() {
    let point_a = Point2D::new(1.0, 2.0);
    let point_b = Point2D::new(1.0, 3.0);
    let point_c = Point2D::new(1.0, 4.0);
    let point_d = Point2D::new(1.0, 2.0);
    let input_set = vec![point_a, point_b, point_c];
    assert_eq!(&point_d, pick_vertex(&input_set));
}
