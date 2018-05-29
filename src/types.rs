// TODO: point2D should accept any integer type. make it generic
// TODO: The generic type should be bound to eq trait
// TODO: add rust doc

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

impl PartialOrd for Fatpoint2D {
    fn partial_cmp(&self, other: &Fatpoint2D) -> Option<Ordering> {
        self.angle.partial_cmp(&other.angle)
    }
}

//implementation methods of Fatpoint2D datatype

//given a set of points, pick the leftmost point
