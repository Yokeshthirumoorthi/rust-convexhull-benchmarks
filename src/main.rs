// use std::fs::File;
// use std::io::prelude::*;

extern crate rustalgo;
// use rustalgo::inputset_gen::*;
use rustalgo::sample_data::*;
use rustalgo::points::*;
use rustalgo::convexhull::*;

fn main() {
    // let mut file = File::create("sample.txt").unwrap();
    // let number_of_vertex = 3;
    // let input_set_10 = get_input_set(10, number_of_vertex);
    // let mut output = String::new();
    // for point in input_set_10 {
    //     output += &format!("({},{}),", point.x, point.y);
    // }
    // file.write_all(output.as_bytes()).unwrap();
    let mut input_set_10: Vec<Point2D> = triangle_10().iter().map(|p| Point2D::new(p.0,p.1)).collect();
    // graham_scan(&mut input_set_10)
    // jarvis_march(&mut input_set_10)
    // chans_algorithm(&mut input_set_10)
    // println!("{:?}", graham_scan(&mut input_set_10))
    // println!("{:?}", jarvis_march(&mut input_set_10))
    // println!("{:?}", chans_algorithm(&mut input_set_10))
}