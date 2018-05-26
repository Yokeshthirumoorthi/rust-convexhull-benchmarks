mod types;
fn main() {
    println!("Hello, world!");
}

//Solves the convex-hull problem by maintaining a stack S
//of candidate points. It pushes each point of the input
//set Q onto the stack one at a time, and it eventually
//pops from the stack each point that is not a vertex of
//CH(Q). When the algorithm terminates, stack S cpntains
//exactly the vertices of CH(Q), in counter clockwise
//order of their appearance on the boundary
fn graham_scan(inputSet: &Vec<types::Point2D>) {
    //panic when inputset has less than or equalto 2 elements
    assert!(inputSet.len() > 2)
}

#[test]
#[should_panic]
fn graham_scan_with_smallinput() {
    let point = types::Point2D::new(1.0, 2.0);
    graham_scan(&vec![point])
}
