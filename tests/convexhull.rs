extern crate rustalgo;
use rustalgo::points::*;
use rustalgo::convexhull::*;

#[test]
#[ignore]
#[should_panic]
fn test_graham_scan_with_smallinput() {
    let point = Point2D::new(1.0, 2.0);
    graham_scan(&vec![point]);
}

#[test]
#[ignore]
fn test_graham_scan() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = graham_scan(&points);
    let h1 = Point2D::new(0.0, 0.0);
    let h2 = Point2D::new(9.0, -9.0);
    let h3 = Point2D::new(9.0, 9.0);
    let hull_should_be = vec![&h1, &h2, &h3];
    assert_eq!(hull, hull_should_be);
}
