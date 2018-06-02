extern crate rustalgo;
use rustalgo::points::*;
use rustalgo::convexhull::*;
use rustalgo::inputset_gen::*;

#[test]
#[should_panic]
fn test_graham_scan_with_smallinput() {
    let point = Point2D::new(1.0, 2.0);
    graham_scan(&mut vec![point]);
}

#[test]
// #[ignore]
fn test_graham_scan() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = graham_scan(&mut points);
    let h1 = Point2D::new(9.0, -9.0);
    let h2 = Point2D::new(9.0, 9.0);
    let h3 = Point2D::new(0.0, 0.0);
    let hull_should_be = vec![h1, h2, h3];
    assert_eq!(hull, hull_should_be);
}

#[test]
// #[ignore]
fn test_jarvis_march() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = jarvis_march(&mut points);
    let h1 = Point2D::new(9.0, -9.0);
    let h2 = Point2D::new(9.0, 9.0);
    let h3 = Point2D::new(0.0, 0.0);
    let hull_should_be = vec![h1, h3, h2];
    assert_eq!(hull, hull_should_be);
}

#[test]
// #[ignore]
fn test_chans_algorithm() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = chans_algorithm(&mut points);
    let h1 = Point2D::new(9.0, -9.0);
    let h2 = Point2D::new(9.0, 9.0);
    let h3 = Point2D::new(0.0, 0.0);
    let hull_should_be = vec![h1, h2, h3];
    assert_eq!(hull, hull_should_be);
}

#[test]
fn test_triangle() {
    // triangle has 3 vertices
    let number_of_vertex = 3;
    
    let vertex_1 = Point2D::new(-1.0000000000000009, -1.7320508075688767);
    let vertex_2 = Point2D::new(2.0, 0.0);
    let vertex_3 = Point2D::new(-0.9999999999999996, 1.7320508075688776);

    let mut input_set_10 = get_input_set(10, number_of_vertex);
    let mut input_set_100 = get_input_set(100, number_of_vertex);
    let mut input_set_10000 = get_input_set(10000, number_of_vertex);
    // let mut input_set_1000000 = get_input_set(1000000, number_of_vertex);
    // let mut input_set_10000000 = get_input_set(10000000, number_of_vertex);

    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_10));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_100));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_10000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_1000000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_10000000));

    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_10));
    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_100));
    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_10000));
    // assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_1000000));
    // assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_10000000));
    // assert_eq!(hull_should_be, chans_algorithm(&mut input_set_10));
}