// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rustalgo;
use rustalgo::points::*;
use rustalgo::convexhull::*;
use rustalgo::convexhull::Algorithm::*;
use rustalgo::inputset::*;
use rustalgo::inputset::Number::*;
use rustalgo::inputset::Shape::*;

#[test]
#[should_panic]
fn test_graham_scan_with_smallinput() {
    let point = Point2D::new(1.0, 2.0);
    graham_scan(&mut vec![point]);
}
#[test]
#[should_panic]
fn test_jarvis_march_with_smallinput() {
    let point = Point2D::new(1.0, 2.0);
    jarvis_march(&mut vec![point]);
}
#[test]
#[should_panic]
fn test_chans_algorithm_with_smallinput() {
    let point = Point2D::new(1.0, 2.0);
    chans_algorithm(&mut vec![point]);
}

fn test_algorithms(algorithm: Algorithm, shape: Shape, hull_should_be: &Vec<Point2D>) {
    let sample_sizes: Vec<Number> = vec![
        Hundred,
        Thousand,
        // TenThousand,
        // HundredThousand,
        // Million,
        // TenMillion,
    ];

    for sample_size in sample_sizes {
        let mut input_set: Vec<Point2D> = generate(shape, sample_size);
        assert_eq!(
            hull_should_be,
            &run_algorithm(algorithm, &mut input_set),
        );
    }
}

fn run_algorithm(algorithm: Algorithm, input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    match algorithm {
        Algorithm::Graham => graham_scan(input_set),
        Algorithm::Jarvis => jarvis_march(input_set),
        Algorithm::Chan => chans_algorithm(input_set),
    }
}

#[test]
fn test_triangle() {
    let vertex_1 = Point2D::new(-1.0000000000000009, -1.7320508075688767);
    let vertex_2 = Point2D::new(2.0, 0.0);
    let vertex_3 = Point2D::new(-0.9999999999999996, 1.7320508075688776);
    let hull_should_be = vec![vertex_1, vertex_2, vertex_3];
    let hull_should_be_jarvis = vec![vertex_1, vertex_3, vertex_2];
    test_algorithms(Graham, Triangle, &hull_should_be);
    test_algorithms(Jarvis, Triangle, &hull_should_be_jarvis);
    test_algorithms(Chan, Triangle, &hull_should_be);
}

#[test]
fn test_rectangle() {
    
    let vertex_1 = Point2D::new(-0.00000000000000036739403974420594, -2.0);
    let vertex_2 = Point2D::new(2.0, 0.0);
    let vertex_3 = Point2D::new(0.00000000000000012246467991473532, 2.0);
    let vertex_4 = Point2D::new(-2.0, 0.00000000000000024492935982947064);

    let hull_should_be = vec![vertex_1, vertex_2, vertex_3, vertex_4];
    let hull_should_be_jarvis = vec![vertex_1, vertex_4, vertex_3, vertex_2];
    test_algorithms(Graham, Rectangle, &hull_should_be);
    test_algorithms(Jarvis, Rectangle, &hull_should_be_jarvis);
    test_algorithms(Chan, Rectangle, &hull_should_be);
}

#[test]
fn test_circle() {
    
    let vertex_1 = Point2D::new(0.34729635533385816, -1.9696155060244165);
    let vertex_2 = Point2D::new(0.999999999999997, -1.732050807568879);
    let vertex_3 = Point2D::new(1.5320888862379534, -1.2855752193730818);
    let vertex_4 = Point2D::new(1.879385241571815, -0.6840402866513421);
    let vertex_5 = Point2D::new(2.0, 0.0);
    let vertex_6 = Point2D::new(1.8793852415718169, 0.6840402866513374);
    let vertex_7 = Point2D::new(1.5320888862379562, 1.2855752193730785);
    let vertex_8 = Point2D::new(1.0000000000000002, 1.7320508075688772);
    let vertex_9 = Point2D::new(0.34729635533386083, 1.969615506024416);
    let vertex_10 = Point2D::new(-0.3472963553338606, 1.969615506024416);
    let vertex_11 = Point2D::new(-0.9999999999999996, 1.7320508075688776);
    let vertex_12 = Point2D::new(-1.5320888862379558, 1.285575219373079);
    let vertex_13 = Point2D::new(-1.8793852415718166, 0.6840402866513378);
    let vertex_14 = Point2D::new(-2.0, 0.00000000000000024492935982947064);
    let vertex_15 = Point2D::new(-1.8793852415718169, -0.6840402866513373);
    let vertex_16 = Point2D::new(-1.5320888862379562, -1.2855752193730785);
    let vertex_17 = Point2D::new(-1.0000000000000009, -1.7320508075688767);
    let vertex_18 = Point2D::new(-0.34729635533386244, -1.9696155060244158);

    let hull_should_be = vec![
        vertex_1, vertex_2, vertex_3, vertex_4, vertex_5, vertex_6, vertex_7, vertex_8, vertex_9,
        vertex_10, vertex_11, vertex_12, vertex_13, vertex_14, vertex_15, vertex_16, vertex_17,
        vertex_18,
    ];
    let hull_should_be_jarvis = vec![
        vertex_1, vertex_18, vertex_17, vertex_16, vertex_15, vertex_14, vertex_13, vertex_12,
        vertex_11, vertex_10, vertex_9, vertex_8, vertex_7, vertex_6, vertex_5, vertex_4, vertex_3,
        vertex_2,
    ];
    test_algorithms(Graham, Circle, &hull_should_be);
    test_algorithms(Jarvis, Circle, &hull_should_be_jarvis);
    test_algorithms(Chan, Circle, &hull_should_be);
}
