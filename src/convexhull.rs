//! Find the convex hull using various methods
use inputset::*;
use points::Point2D;

/// Solve the convexhull problem using Graham-Scan
///
///This method solves the convex-hull by maintaining a stack S
///of candidate points. It pushes each point of the input
///set Q onto the stack one at a time, and it eventually
///pops from the stack each point that is not a vertex of
///CH(Q). When the algorithm terminates, stack S cpntains
///exactly the vertices of CH(Q), in counter clockwise
///order of their appearance on the boundary.
pub fn graham_scan<'a>(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //find the pivot point in the input set with the
    //minimum y-coordinate, or the leftmost such point
    //in case of tie and set the pivot point as first
    //element of the set
    set_pivot(input_set);

    //sort the remianing elements in input set by polar
    //angle in counter clockwise order around pivot point.
    //(if more than one point has the same angle, remove all
    //but the one that is farthest from pivot point)
    let sorted_input_set = sort_polar_angle_ccw(input_set);

    //panic when input_set has less than or equalto 2 elements
    assert!(input_set.len() > 2);

    //initialize the stack that will maintain the candidate points
    let mut hull_points: Vec<Point2D> = Vec::new();
    hull_points.push(sorted_input_set[0]);
    hull_points.push(sorted_input_set[1]);
    hull_points.push(sorted_input_set[2]);
    for i in 3..sorted_input_set.len() {
        while !hull_points[hull_points.len() - 2]
            .ccw(&hull_points[hull_points.len() - 1], &sorted_input_set[i])
        {
            hull_points.pop();
        }
        hull_points.push(sorted_input_set[i])
    }
    hull_points
}
