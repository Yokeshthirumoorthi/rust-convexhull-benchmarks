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
    // println!("{:?}", input_set);
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
    fat_pt_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // println!("{:?}", fat_pt_vec);
    //convert back to point2d
    let mut output = Vec::new();
    if let Some(first) = input_set.get(0) {
        output.push(Fatpoint2D::new(first, &first).to_point())
    }
    for fat_point in fat_pt_vec.iter() {
        output.push(fat_point.to_point());
    }
    // println!("Line Polar Angle:{:?}\n", output);
    output
}

// [
// Point2D { x: 9.0, y: -9.0 }
// , Point2D { x: 1.0, y: 1.0 }
// , Point2D { x: 1.0, y: 0.0 }
// , Point2D { x: 2.0, y: 2.0 }
// , Point2D { x: 1.0, y: -1.0 }
// , Point2D { x: 2.0, y: 0.0 }
// , Point2D { x: 3.0, y: 3.0 }
// , Point2D { x: 2.0, y: -2.0 }
// , Point2D { x: 3.0, y: 0.0 }
// , Point2D { x: 4.0, y: 4.0 }
// , Point2D { x: 3.0, y: -3.0 }
// , Point2D { x: 4.0, y: 0.0 }
// , Point2D { x: 5.0, y: 5.0 }
// , Point2D { x: 4.0, y: -4.0 }
// , Point2D { x: 5.0, y: 0.0 }
// , Point2D { x: 6.0, y: 6.0 }
// , Point2D { x: 5.0, y: -5.0 }
// , Point2D { x: 6.0, y: 0.0 }
// , Point2D { x: 7.0, y: 7.0 }
// , Point2D { x: 6.0, y: -6.0 }
// , Point2D { x: 7.0, y: 0.0 }
// , Point2D { x: 8.0, y: 8.0 }
// , Point2D { x: 7.0, y: -7.0 }
// , Point2D { x: 8.0, y: 0.0 }
// , Point2D { x: 9.0, y: 9.0 }
// , Point2D { x: 8.0, y: -8.0 }
// , Point2D { x: 9.0, y: 0.0 }
// , Point2D { x: 0.0, y: 0.0 }
// ]

// [
// Fatpoint2D { x: 9.0, y: 9.0, distance: 18.0, angle: -1.5707963267948966 }, 
// Fatpoint2D { x: 9.0, y: 0.0, distance: 9.0, angle: -1.5707963267948966 }, 
// Fatpoint2D { x: 8.0, y: 8.0, distance: 17.029386365926403, angle: -1.512040504079174 }, 
// Fatpoint2D { x: 8.0, y: 0.0, distance: 9.055385138137417, angle: -1.460139105621001 }, 
// Fatpoint2D { x: 7.0, y: 7.0, distance: 16.1245154965971, angle: -1.446441332248135 }, 
// Fatpoint2D { x: 6.0, y: 6.0, distance: 15.297058540778355, angle: -1.373400766945016 }, 
// Fatpoint2D { x: 7.0, y: 0.0, distance: 9.219544457292887, angle: -1.3521273809209546 }, 
// Fatpoint2D { x: 5.0, y: 5.0, distance: 14.560219778561036, angle: -1.2924966677897853 }, 
// Fatpoint2D { x: 6.0, y: 0.0, distance: 9.486832980505138, angle: -1.2490457723982544 }, 
// Fatpoint2D { x: 4.0, y: 4.0, distance: 13.92838827718412, angle: -1.2036224929766774 }, 
// Fatpoint2D { x: 5.0, y: 0.0, distance: 9.848857801796104, angle: -1.1525719972156676 }, 
// Fatpoint2D { x: 3.0, y: 3.0, distance: 13.416407864998739, angle: -1.1071487177940904 }, 
// Fatpoint2D { x: 4.0, y: 0.0, distance: 10.295630140987, angle: -1.0636978224025597 }, 
// Fatpoint2D { x: 2.0, y: 2.0, distance: 13.038404810405298, angle: -1.0040671092713902 }, 
// Fatpoint2D { x: 3.0, y: 0.0, distance: 10.816653826391969, angle: -0.982793723247329 }, 
// Fatpoint2D { x: 2.0, y: 0.0, distance: 11.40175425099138, angle: -0.9097531579442097 }, 
// Fatpoint2D { x: 1.0, y: 1.0, distance: 12.806248474865697, angle: -0.8960553845713439 }, 
// Fatpoint2D { x: 1.0, y: 0.0, distance: 12.041594578792296, angle: -0.844153986113171 }, 
// Fatpoint2D { x: 1.0, y: -1.0, distance: 11.313708498984761, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 2.0, y: -2.0, distance: 9.899494936611665, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 3.0, y: -3.0, distance: 8.48528137423857, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 4.0, y: -4.0, distance: 7.0710678118654755, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 5.0, y: -5.0, distance: 5.656854249492381, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 6.0, y: -6.0, distance: 4.242640687119285, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 7.0, y: -7.0, distance: 2.8284271247461903, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 8.0, y: -8.0, distance: 1.4142135623730951, angle: -0.7853981633974483 }, 
// Fatpoint2D { x: 0.0, y: 0.0, distance: 12.727922061357855, angle: -0.7853981633974483 }
// ]

// [
// Point2D { x: 9.0, y: -9.0 }, 
// Point2D { x: 1.0, y: 1.0 }, 
// Point2D { x: 2.0, y: 2.0 }, 
// Point2D { x: 3.0, y: 3.0 }, 
// Point2D { x: 4.0, y: 4.0 }, 
// Point2D { x: 5.0, y: 5.0 }, 
// Point2D { x: 6.0, y: 6.0 }, 
// Point2D { x: 7.0, y: 7.0 }, 
// Point2D { x: 8.0, y: 8.0 }, 
// Point2D { x: 9.0, y: 9.0 }, 
// Point2D { x: 9.0, y: 0.0 }, 
// Point2D { x: 0.0, y:0.0 }
// ]