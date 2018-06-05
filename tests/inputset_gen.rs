// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rustalgo;
use rustalgo::inputset_gen::*;

#[test]
fn test_generate() {
    assert_eq!(36, get_input_set(36, 3).len());
}
