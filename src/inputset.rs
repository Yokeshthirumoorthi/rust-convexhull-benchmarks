//! Collection of functions that operates and
//! modifies the input set to conveniently
//! determine the hull points
use points::*;

///Converts any raw point tuple to Point2D type
/// and pushes it to the inputset
pub fn push(inputset: &mut Vec<Point2D>, point: (f64, f64)) -> &Vec<Point2D> {
    let point = Point2D::new(point.0, point.1);
    inputset.push(point);
    inputset
}

///Finds the point in input set with least y-coordinate or the
///left most in case of a match
/// Sets the pivot point of the inputset
/// as the first element of the input set.
pub fn set_pivot(input_set: &mut Vec<Point2D>) -> &Vec<Point2D> {
    //panic if there are no elements in the input_set
    assert!(input_set.len() > 0);
    //set the vertex point to be the
    //first element of the inputset
    for i in 0..input_set.len() {
        if input_set[i].pick_left(&input_set[0]) == &input_set[i] {
            &input_set.swap(0, i);
        }
    }
    input_set
}

///Sorts the  elements of input set by polar
///angle in counter clockwise order around pivot point.
///(if more than one point has the same angle, remove all
///but the one that is farthest from pivot point)
pub fn sort_polar_angle_ccw(input_set: &Vec<Point2D>) -> Vec<Point2D> {
    let mut fat_pt_vec: Vec<Fatpoint2D> = Vec::new();
    //convert all the point2D as FatPoints
    if let Some((first, elements)) = input_set.split_first() {
        fat_pt_vec = elements.iter()
                        .map(|point| Fatpoint2D::new(point, &first))
                        .collect();
    };                     
    //sort the fatpoint vec
    fat_pt_vec.sort_by(|a, b| b.partial_cmp_distance(a).unwrap());
    fat_pt_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    fat_pt_vec.dedup_by(|a, b| a.eq_polar_angle(b));
    //convert back to point2d
    let mut output = Vec::new();
    if let Some(first) = input_set.get(0) {
        output.push(Fatpoint2D::new(first, &first).to_point())
    }
    for fat_point in fat_pt_vec.iter() {
        output.push(fat_point.to_point());
    }
    output
}

pub fn jarvis_binary_search(next_to_top: &Point2D, top: &Point2D, sub_hull_set: &Vec<Point2D>) -> Point2D {
    let mut max_angled_point = sub_hull_set[0];
    let mut max_angle = 0.0;
    for point in sub_hull_set {
        let orientation = orietation(&next_to_top, &top, &point);
        if max_angle < orientation {
            max_angle = orientation;
            max_angled_point = *point
        }
    }
    max_angled_point
}

// Splits the input set into chunks
// 
// Mostly there is a higher change for the size of last chunk to be 
// less than or equal to 2 elements. This function ensures that
// no chunk has less than or equal to 2 elements.
//  
// #Panics
// Panics if m is less than size of actual input set
// or m is less than 3

// pub fn split_by_m(m: usize, actual_input: &mut Vec<Point2D>) -> Vec<Vec<Point2D>> {
//     assert!(m < actual_input.len() && m > 3);
//     let mut output: Vec<Vec<Point2D>> = Vec::new();

//     let total_number_of_chunks = actual_input.len() / m;
//     let size_of_last_set = actual_input.len() - (total_number_of_chunks * m);
//     // println!("size_of_last_set: {}", size_of_last_set);
//     // for _ in 0..total_number_of_chunks {    
//     //         let mut chunk: Vec<Point2D> = Vec::new();
//     //         for _ in 0..m {
//     //             chunk.push(actual_input[0]);
//     //             actual_input.remove(0);
//     //         }
//     //         output.push(chunk);
//     // }
    
//     // if size_of_last_set == 0 || size_of_last_set > 2 {
        
//     // }

//     output
// }