// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rustalgo;
use rustalgo::points::*;
use rustalgo::inputset::*;

#[test]
#[should_panic]
fn test_set_pivot_with_emptyinput() {
    set_pivot(&mut Vec::new());
}

#[test]
fn test_set_pivot() {
    let point_a = Point2D::new(1.0, 2.0);
    let point_b = Point2D::new(1.0, 3.0);
    let point_c = Point2D::new(1.0, 0.0);
    let point_a1 = Point2D::new(1.0, 2.0);
    let point_b1 = Point2D::new(1.0, 3.0);
    let point_c1 = Point2D::new(1.0, 0.0);
    let mut input_set = vec![point_a, point_b, point_c];
    let out_set = vec![point_c1, point_b1, point_a1];
    assert_eq!(&out_set, set_pivot(&mut input_set));
}

#[test]
fn test_sort_polar_angle_ccw() {
    let point_a = Point2D::new(1.0, 0.0);
    let point_b = Point2D::new(2.0, 2.0);
    let point_c = Point2D::new(3.0, 1.0);
    let point_a1 = Point2D::new(1.0, 0.0);
    let point_b1 = Point2D::new(2.0, 2.0);
    let point_c1 = Point2D::new(3.0, 1.0);
    let mut input_set = vec![point_a, point_b, point_c];
    let out_set = vec![point_a1, point_c1, point_b1];
    assert_eq!(out_set, sort_polar_angle_ccw(&mut input_set));
}
