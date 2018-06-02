//! Find the convex hull using various methods
use inputset::*;
use points::Point2D;

/// Solves the convexhull problem using Graham-Scan
///
/// This method solves the convex-hull by maintaining a stack S
/// of candidate points. It pushes each point of the input
/// set Q onto the stack one at a time, and it eventually
/// pops from the stack each point that is not a vertex of
/// CH(Q). When the algorithm terminates, stack S cpntains
/// exactly the vertices of CH(Q), in counter clockwise
/// order of their appearance on the boundary.
/// 
/// The psedocode for this algorithm is referred from
/// Introduction to Algorithms (Third Edition)
/// Authors: Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest and Clifford Stein
pub fn graham_scan(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
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

/// Solves the convexhull problem using Jarvis-March
///
/// The psedocode for this algorithm is referred from
/// Introduction to Algorithms (Third Edition)
/// Authors: Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest and Clifford Stein
pub fn jarvis_march(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
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

    let mut last_known_hull_point = sorted_input_set[0];
    //initialize the stack that will maintain the candidate points
    let mut hull_points: Vec<Point2D> = Vec::new();
    hull_points.push(last_known_hull_point);
    for _i in 0..sorted_input_set.len() {
        let p_i = last_known_hull_point;
        let mut end_point = sorted_input_set[0];
        for j in 1..sorted_input_set.len() {
            if end_point == last_known_hull_point || sorted_input_set[j].ccw(&p_i, &end_point) {
                end_point = sorted_input_set[j];
            }
        }
        if end_point == hull_points[0] {
            break;
        }
        hull_points.push(end_point);
        last_known_hull_point = end_point;
    }
    hull_points
}

/// Solves the convexhull problem using chans-algorithm
///
/// The pseudocode for this algorithm is referred from
/// https://en.wikipedia.org/wiki/Chan%27s_algorithm
/// https://www.slideshare.net/amrinderarora/convex-hull-chans-algorithm-on-log-h-output-sensitive-algorithm
pub fn chans_algorithm(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    set_pivot(input_set);
    let p_1 = input_set[0];
    let p_0 = Point2D::new(-10.0, -10.0);
    for t in 1..(input_set.len() as f64).log2().log2().ceil() as u32 {
        let m = 2_i32.pow(2).pow(t) as usize;
        let mut q_k = input_set.chunks(m);
        let mut c_k: Vec<Vec<Point2D>> = Vec::new();
        for k in q_k {
            c_k.push(graham_scan(&mut k.to_vec()));
        }
        let mut hull_points: Vec<Point2D> = Vec::new();
        hull_points.push(p_1);
        for i in 0..(m - 1) {
            let mut q_i_k: Vec<Point2D> = Vec::new();
            for k in &c_k {
                if i == 0 {
                    q_i_k.push(jarvis_binary_search(&p_0, &hull_points[i], k));
                } else {
                    q_i_k.push(jarvis_binary_search(&hull_points[i-1], &hull_points[i], k));
                }
            }
            // let next_hull_point = hull_points[0];
            let next_hull_point = if i == 0 {
                jarvis_binary_search(&p_0, &hull_points[i], &q_i_k)
            } else {
                jarvis_binary_search(&hull_points[i-1], &hull_points[i], &q_i_k)
            };
            if next_hull_point == hull_points[0] {
                return hull_points;
            } else {
                hull_points.push(next_hull_point);
            }
        }
    }
    Vec::new()
}