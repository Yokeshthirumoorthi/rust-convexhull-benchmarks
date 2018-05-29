use points::*;

pub fn push(inputset: &mut Vec<Point2D>, point: (f64, f64)) -> &Vec<Point2D> {
    let point = Point2D::new(point.0, point.1);
    inputset.push(point);
    inputset
}

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

pub fn set_pivot(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //TODO: implement the functionality
    Vec::new()
}

pub fn sort_by_polar_angle_ccw(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //TODO: implement the functionality
    Vec::new()
}
