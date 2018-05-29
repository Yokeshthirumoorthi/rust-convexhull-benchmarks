//! Points types for finding canvex hull

///A basic representation of a point
///
///With x and y coordinate, a point2D
///is a point in 2D euclidean space
///
#[derive(Debug, Copy, Clone)]
pub struct Point2D {
    /// x-coordinate value
    x: f64,
    /// y-coordinate value
    y: f64,
}

impl PartialEq for Point2D {
    /// compare 2 points using = sign
    /// and return true when both x and y
    /// coordinate are same
    fn eq(&self, other: &Point2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

///Implementation methods for Point2D type
///
/// Some handy methods find the convex hull
/// using the point
impl Point2D {
    ///Constructor for Point2D
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }

    ///Comparision of point position relative to another point
    ///
    ///Given two points we need to left most point.
    /// This is used to find the pivot point of the vertex point
    /// of the hull.
    pub fn pick_left<'a>(&'a self, other: &'a Point2D) -> &'a Point2D {
        //when both the points are same, return the other point
        if self == other {
            return other;
        } else if self.y != other.y {
            // else return the point with min y-coordinate
            if self.y < other.y {
                return self;
            } else {
                return other;
            }
        } else {
            // when y-coordinates are same return the point with min x-coordinate
            if self.x < other.x {
                return self;
            } else {
                return other;
            }
        }
    }

    ///Determine the turn direction around the corner
    /// formed by the points a, b and c.
    ///
    /// Return true for counterclockwise turn
    /// and false for colinearity or clockwise turns.
    ///
    pub fn ccw(&self, point_b: &Point2D, point_c: &Point2D) -> bool {
        (point_b.x - self.x) * (point_c.y - self.y) - (point_b.y - self.y) * (point_c.x - self.x)
             > 0.0
    }

    ///Determine the distance between 2 points
    fn compute_distance(&self, point2: &Point2D) -> f64 {
        ((self.x - point2.x).powi(2) + (self.y - point2.y).powi(2)).sqrt()
    }

    ///Determine the polarangle between 2 points
    fn compute_angle(&self, point2: &Point2D) -> f64 {
        (point2.y - self.y).atan2(point2.x - self.x)
    }
}

///An extended representation of a point
///
/// We store additional information to a
/// point2d with respect to the vertex point.
/// Using this information we could sort the points
/// in the input set.
///
#[derive(Debug, PartialEq)]
pub struct Fatpoint2D {
    x: f64,
    y: f64,
    distance: f64,
    angle: f64,
}

use std::cmp::Ordering;
impl PartialOrd for Fatpoint2D {
    /// A handy method to sort points based on their angle
    /// and distance from the vertex point
    fn partial_cmp(&self, other: &Fatpoint2D) -> Option<Ordering> {
        self.angle.partial_cmp(&other.angle)
    }
}

impl Fatpoint2D {
    /// create properties for a point from another point usually the vertex
    pub fn new(point: &Point2D, vertex: &Point2D) -> Fatpoint2D {
        Fatpoint2D {
            x: point.x,
            y: point.y,
            distance: point.compute_distance(&vertex),
            angle: point.compute_angle(&vertex),
        }
    }
    
    /// canverts a fatpoint back to point2d
    pub fn to_point(&self) -> Point2D {
        Point2D::new(self.x, self.y)
    }

    pub fn partial_cmp_distance(&self, other: &Fatpoint2D) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }

    pub fn eq_polar_angle(&self, other: &Fatpoint2D) -> bool {
        self.angle == other.angle
    }
}

//some test cases for point2D data type.
#[test]
fn test_new_2d_point() {
    assert_eq!(Point2D { x: 1.0, y: 2.0 }, Point2D::new(1.0, 2.0));
}

#[test]
fn test_new_fat_point() {
    let point_a = Point2D::new(1.0, 2.0);
    let point_b = Point2D::new(1.0, 3.0);
    assert_eq!(
        Fatpoint2D {
            x: 1.0,
            y: 2.0,
            distance: 1.0,
            angle: 1.5707963267948966,
        },
        Fatpoint2D::new(&point_a, &point_b)
    );
}

#[test]
fn test_fat_point_cmp() {
    let fat_point_a = Fatpoint2D {
        x: 1.0,
        y: 2.0,
        distance: 10.0,
        angle: 10.0,
    };
    let fat_point_b = Fatpoint2D {
        x: 1.0,
        y: 3.0,
        distance: 10.0,
        angle: 1.0,
    };
    let mut set = vec![&fat_point_a, &fat_point_b];
    set.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec![&fat_point_b, &fat_point_a], set);
}
