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

    //given other point pick the left point
    //first compare the y-coordinate, if it
    //is same then compare x-coordinate.
    //when both the points are same return any one
    pub fn pickleft(&self, other: &Point2D) -> &Point2D {
        if self == other {
            return &self;
        }
        if self.y != other.y {
            if self.y < other.y {
                return &self;
            } else {
                return &other;
            }
        }
        if self.x != other.x {
            if self.x < other.x {
                return &self;
            } else {
                return &other;
            }
        }
    }
}
