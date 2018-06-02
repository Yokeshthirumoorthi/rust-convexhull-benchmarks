extern crate rustalgo;
use rustalgo::inputset_gen::*;

#[test]
fn test_generate() {
    assert_eq!(36, get_input_set(36, 3).len());
}
