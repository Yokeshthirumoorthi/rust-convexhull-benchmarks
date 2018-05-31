extern crate rustalgo;
use rustalgo::rand_gen::*;

extern crate rand;
use rand::distributions::{Range};


#[test]
fn test_generate() {
    assert_eq!(2, generate(2500, Range::new(-1_f64, 1.), Shape::Circle).len());
}
