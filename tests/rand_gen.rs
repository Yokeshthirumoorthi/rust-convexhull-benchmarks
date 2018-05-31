extern crate rustalgo;
use rustalgo::rand_gen::*;

extern crate rand;
use rand::distributions::{Range};


#[test]
fn test_generate() {
    assert_eq!(25, generate(25, Range::new(-1_f64, 1.), Shape::Circle).len());
}
