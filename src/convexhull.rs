//! Find the convex hull using various methods
use points::Point2D;

/// Solve the convexhull problem using Graham-Scan
///
///This method solves the convex-hull by maintaining a stack S
///of candidate points. It pushes each point of the input
///set Q onto the stack one at a time, and it eventually
///pops from the stack each point that is not a vertex of
///CH(Q). When the algorithm terminates, stack S cpntains
///exactly the vertices of CH(Q), in counter clockwise
///order of their appearance on the boundary
pub fn graham_scan<'a>(input_set: &Vec<Point2D>) -> Vec<&Point2D> {
    //panic when input_set has less than or equalto 2 elements
    assert!(input_set.len() > 2);
    //initialize the stack that will maintain the candidate points
    let mut s: Vec<&Point2D> = Vec::new();
    s.push(&input_set[0]);
    s.push(&input_set[1]);
    s.push(&input_set[2]);
    for i in 3..input_set.len() {
        while s[s.len() - 2].ccw(&s[s.len() - 1], &input_set[i]) {
            s.pop();
        }
        s.push(&input_set[i])
    }
    s
}
