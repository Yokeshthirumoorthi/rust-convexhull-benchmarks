// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rustalgo;
use rustalgo::points::*;

#[test]
fn test_pick_left() {
    let point_a = Point2D::new(1.0, 2.0);
    let point_b = Point2D::new(1.0, 3.0);
    let point_c = Point2D::new(0.0, 2.0);
    assert_eq!(&point_a, point_a.pick_left(&point_a));
    assert_eq!(&point_a, point_a.pick_left(&point_b));
    assert_eq!(&point_c, point_a.pick_left(&point_c));
}

#[test]
fn test_ccw() {
    let point_a = Point2D::new(1.0, 1.0);
    let point_b = Point2D::new(2.0, 2.0);
    let point_c = Point2D::new(3.0, 2.5);
    assert_eq!(false, point_a.ccw(&point_b, &point_c));
    let point_a = Point2D::new(0.0, 0.0);
    let point_b = Point2D::new(1.0, 1.0);
    let point_c = Point2D::new(2.0, 0.0);
    assert_eq!(true, point_a.ccw(&point_c, &point_b));
}
