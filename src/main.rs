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
fn graham_scan<'a>(input_set: &Vec<&'a types::Point2D>) -> Vec<&'a types::Point2D> {
    //panic when input_set has less than or equalto 2 elements
    assert!(input_set.len() > 2);
    //initialize the stack that will maintain the candidate points
    let mut s: Vec<&types::Point2D> = Vec::new();
    s.push(&input_set[0]);
    s.push(&input_set[1]);
    s.push(&input_set[2]);
    for i in 3..input_set.len() {
        s.push(&input_set[i])
    }
    s
}

// #[test]
// #[should_panic]
// fn test_graham_scan_with_smallinput() {
//     let point = types::Point2D::new(1.0, 2.0);
//     graham_scan(&vec![point])
// }

#[test]
fn test_graham_scan() {
    let point1 = types::Point2D::new(1.0, 2.0);
    let point2 = types::Point2D::new(1.0, 3.0);
    let point3 = types::Point2D::new(1.0, 4.0);
    let point4 = types::Point2D::new(1.0, 5.0);
    let point5 = types::Point2D::new(1.0, 6.0);
    assert_eq!(
        vec![&point1, &point2, &point3, &point4, &point5],
        graham_scan(&vec![&point1, &point2, &point3, &point4, &point5])
    )
}
