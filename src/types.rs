// TODO: point2D should accept any integer type. make it generic
// TODO: The generic type should be bound to eq trait
// TODO: add rust doc
use std::cmp::Ordering;

//point2D data type with x and y coordinate values
//is the basic representation of a point in
//2d euclidean space
#[derive(Debug)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Point2D) -> bool {
        self.x == other.x && self.y == other.y
    }
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

//a type for storing additional properties of a point
//derived from the vertex point.
//These details are used for sorting points based on vertex
#[derive(Debug, PartialEq)]
pub struct Fatpoint2D {
    x: f64,
    y: f64,
    distance: f64,
    angle: f64,
}

// TODO: this function is private only for fatpoint2d implementation
//compute euclidean distance between 2 points
fn compute_distance(point1: &Point2D, point2: &Point2D) -> f64 {
    ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt()
}
// TODO: this function is private only for fatpoint2d implementation
//compute polar angle between 2 points
fn compute_angle(point1: &Point2D, point2: &Point2D) -> f64 {
    (point2.y - point1.y).atan2((point2.x - point1.x))
}

impl PartialOrd for Fatpoint2D {
    fn partial_cmp(&self, other: &Fatpoint2D) -> Option<Ordering> {
        self.angle.partial_cmp(&other.angle)
    }
}

//implementation methods of Fatpoint2D datatype
impl Fatpoint2D {
    //create properties for a point from another point usually the vertex
    fn new(point: &Point2D, vertex: &Point2D) -> Fatpoint2D {
        Fatpoint2D {
            x: point.x,
            y: point.y,
            distance: compute_distance(point, vertex),
            angle: compute_angle(point, vertex),
        }
    }

    // //provide cmp function to sortby angle
    // fn cmp_fatpoint2D(&self, point2: &Fatpoint2D) {
    //     self.angle.partial_cmp(&point2.angle).unwrap();
    // }
}

//some test cases for point2D data type.
#[test]
fn test_add_new_points() {
    assert_eq!(Point2D { x: 1.0, y: 2.0 }, Point2D::new(1.0, 2.0));
}
#[test]
fn test_add_new_points_details() {
    let pointA = Point2D::new(1.0, 2.0);
    let pointB = Point2D::new(1.0, 3.0);
    assert_eq!(
        Fatpoint2D {
            x: 1.0,
            y: 2.0,
            distance: 1.0,
            angle: 1.5707963267948966,
        },
        Fatpoint2D::new(&pointA, &pointB)
    );
}
#[test]
fn test_fat_pt_cmp() {
    let fat_pointA = Fatpoint2D {
        x: 1.0,
        y: 2.0,
        distance: 10.0,
        angle: 10.0,
    };
    let fat_pointB = Fatpoint2D {
        x: 1.0,
        y: 3.0,
        distance: 10.0,
        angle: 1.0,
    };
    let mut set = vec![&fat_pointA, &fat_pointB];
    set.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec![&fat_pointB, &fat_pointA], set);
}

#[test]
fn test_pick_left() {
    let pointA = Point2D::new(1.0, 2.0);
    let pointB = Point2D::new(1.0, 3.0);
    let pointC = Point2D::new(0.0, 2.0);
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
    let pointA = Point2D::new(1.0, 2.0);
    let pointB = Point2D::new(1.0, 3.0);
    let pointC = Point2D::new(1.0, 4.0);
    let pointD = Point2D::new(1.0, 2.0);
    let input_set = vec![pointA, pointB, pointC];
    assert_eq!(&pointD, pick_vertex(&input_set));
}
