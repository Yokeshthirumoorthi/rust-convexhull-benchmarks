//point2D data type with x and y coordinate values
//is the basic representation of a point in
//2d euclidean space
#[derive(Debug, Eq, PartialEq)]
pub struct Point2D {
    x: u64,
    y: u64,
}

//implement methods of point2D datatype
impl Point2D {
    // a handy method to create new points
    pub fn new(x: u64, y: u64) -> Point2D {
        Point2D { x, y }
    }

    //given other point pick the left point
    //first compare the y-coordinate, if it
    //is same then compare x-coordinate.
    //when both the points are same return any one
    pub fn pickleft<'a>(&'a self, other: &'a Point2D) -> &'a Point2D {
        if self == other {
            return other;
        }
        if self.y != other.y {
            if self.y < other.y {
                return self;
            } else {
                return other;
            }
        } else {
            if self.x < other.x {
                return self;
            } else {
                return other;
            }
        }
    }
}

//some test cases for point2D data type.
#[test]
fn test_add_new_points() {
    assert_eq!(Point2D { x: 1, y: 2 }, Point2D::new(1, 2));
}
#[test]
fn test_pick_left() {
    let pointA = Point2D::new(1, 2);
    let pointB = Point2D::new(1, 3);
    let pointC = Point2D::new(0, 2);
    assert_eq!(&pointA, pointA.pickleft(&pointA));
    assert_eq!(&pointA, pointA.pickleft(&pointB));
    assert_eq!(&pointC, pointA.pickleft(&pointC));
}

//given a set of points, pick the leftmost point
fn pick_vertex(input_set: &Vec<Point2D>) -> &Point2D {
    //panic if there are no elements in the input_set
    assert!(input_set.len() > 0);
    //initialize the vertex point to be the first point in input_set
    let mut vertex_point = &input_set[0];
    for point in input_set {
        vertex_point = &point.pickleft(vertex_point);
    }
    vertex_point
}

#[test]
fn test_add_pick_vertex() {
    let pointA = Point2D::new(1, 2);
    let pointB = Point2D::new(1, 3);
    let pointC = Point2D::new(1, 4);
    let pointD = Point2D::new(1, 2);
    let input_set = vec![pointA, pointB, pointC];
    assert_eq!(&pointD, pick_vertex(&input_set));
}
//TODO:
// point2D should accept any integer type. make it generic
// The generic type should be bound to eq trait
// add rust doc
