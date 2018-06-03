use std::fs::File;
use std::io::prelude::*;

extern crate rustalgo;
use rustalgo::inputset_gen::*;

fn main() {
    let mut file = File::create("sample.txt").unwrap();
    let number_of_vertex = 3;
    let input_set_10 = get_input_set(10, number_of_vertex);
    let mut output = String::new();
    for point in input_set_10 {
        output += &format!("({},{}),", point.x, point.y);
    }
    file.write_all(output.as_bytes()).unwrap();
}