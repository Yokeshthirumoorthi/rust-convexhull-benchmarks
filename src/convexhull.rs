// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Find the convex hull using various methods
use inputset::*;
use points::Point2D;

///Types of algorithms handled in this programm
#[derive(Debug, Copy, Clone)]
pub enum Algorithm {
    Graham,
    Jarvis,
    Chan,
}

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
    let sorted_input_set = input_set;

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
    let sorted_input_set = input_set;

    let mut last_known_hull_point = sorted_input_set[0];
    //initialize the stack that will maintain the candidate points
    let mut hull_points: Vec<Point2D> = Vec::new();
    hull_points.push(last_known_hull_point);
    for _i in 0..sorted_input_set.len() {
        let p_i = last_known_hull_point;
        let mut end_point = sorted_input_set[0];
        for j in 1..sorted_input_set.len() {
            if end_point == last_known_hull_point 
                            || sorted_input_set[j].ccw(&p_i, &end_point) {
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
    let sorted_input_set = input_set;   

    let first_hull_vertex = sorted_input_set[0];
    //assign a least element (-infinity, 0). This is used
    //for finding second hull element, when we have only one
    //known hull vertex in the output set.
    //Note: point_off is not a point of input_set
    let point_off = Point2D::new(first_hull_vertex.x - 1.0, 0.);

    //t is the iteration needed to find m.
    //m is the magic number of chans algorithm. It is required to
    //be greater than or equal to h (number of vertices)
    for t in 1..sorted_input_set.len() {
        let mut m = 2_i32.pow(2).pow(t as u32) as usize;
        // split the input into smaller chunks using m
        let mut total_number_of_chunks = sorted_input_set.len() / m;
        //ensure that the last chunk has atleat 3 elements in it.
        //else the algorithm will panic.
        let mut size_of_last_set 
                    = sorted_input_set.len() - (total_number_of_chunks * m);
        while size_of_last_set > 0 && size_of_last_set < 3 {
            m += 1;
            total_number_of_chunks = sorted_input_set.len() / m;
            size_of_last_set 
                    = sorted_input_set.len() - (total_number_of_chunks * m);
        }

        let mut chunks_set = sorted_input_set.chunks(m);
        let mut hull_set_of_chuncks: Vec<Vec<Point2D>> = Vec::new();
        for chunk_set in chunks_set {
            let mut sorted_chunk = sort_input(&mut chunk_set.to_vec());
            hull_set_of_chuncks.push(graham_scan(&mut sorted_chunk));
        }
        let mut hull_points: Vec<Point2D> = Vec::new();
        hull_points.push(first_hull_vertex);
        // Find 1 hull point in each iteration.
        // If not all the hull points are found within m iterations
        // reset m and restart the algorithm.
        for i in 0..(m - 1) {
            let mut hull_candidates_of_chuncks: Vec<Point2D> = Vec::new();
            //pick candidates from each chunk
            for hull_set_of_chunck in &hull_set_of_chuncks {
                hull_candidates_of_chuncks.push(jarvis_binary_search(
                    if hull_points.len() > 1 {
                        &hull_points[i - 1]
                    } else {
                        &point_off
                    },
                    &hull_points[i],
                    hull_set_of_chunck,
                ));
            }
            //find the next hull point from the chosen candidates
            let next_hull_point = jarvis_binary_search(
                if hull_points.len() > 1 {
                    &hull_points[i - 1]
                } else {
                    &point_off
                },
                &hull_points[i],
                &hull_candidates_of_chuncks,
            );
            //iterate or end
            if next_hull_point == hull_points[0] {
                return hull_points;
            } else {
                hull_points.push(next_hull_point);
            }
        }
    }
    Vec::new()
}

/// Prepares the input set for executing the algorithm.
/// 
/// Finds the first hull element and sorts the rest 
/// of the elements based on angle formed with the first
/// hull element
/// 
/// # Panics
/// Panics when size of input is not atleast 3
/// 
pub fn sort_input(input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //panic when input_set has less than or equalto 2 elements
    assert!(input_set.len() > 2);

    //find the pivot point in the input set with the
    //minimum y-coordinate, or the leftmost such point
    //in case of tie and set the pivot point as first
    //element of the set
    set_pivot(input_set);

    //sort the remianing elements in input set by polar
    //angle in counter clockwise order around pivot point.
    //(if more than one point has the same angle, remove all
    //but the one that is farthest from pivot point)
    sort_polar_angle_ccw(input_set)
}

/// Executes an algorithm for given inputset of point and returns the hull points
pub fn execute(algorithm: Algorithm, input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    match algorithm {
        Algorithm::Graham => graham_scan(input_set),
        Algorithm::Jarvis => jarvis_march(input_set),
        Algorithm::Chan => chans_algorithm(input_set),
    }
}