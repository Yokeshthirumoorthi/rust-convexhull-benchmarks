//point2D data type with x and y coordinate values
//is the basic representation of a point in
//2d euclidean space
pub struct Point2D {
    x: f64,
    y: f64,
}

//implement methods of point2D datatype
impl Point2D {
    // a handy method to create new points
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }
}
