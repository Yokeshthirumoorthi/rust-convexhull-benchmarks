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
