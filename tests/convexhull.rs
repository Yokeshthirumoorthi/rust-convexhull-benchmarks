extern crate rustalgo;
use rustalgo::points::*;
use rustalgo::convexhull::*;
use rustalgo::inputset_gen::*;

#[test]
#[should_panic]
fn test_graham_scan_with_smallinput() {
    let point = Point2D::new(1.0, 2.0);
    graham_scan(&mut vec![point]);
}

#[test]
// #[ignore]
fn test_graham_scan() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = graham_scan(&mut points);
    let h1 = Point2D::new(9.0, -9.0);
    let h2 = Point2D::new(9.0, 9.0);
    let h3 = Point2D::new(0.0, 0.0);
    let hull_should_be = vec![h1, h2, h3];
    assert_eq!(hull, hull_should_be);
}

#[test]
// #[ignore]
fn test_jarvis_march() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = jarvis_march(&mut points);
    let h1 = Point2D::new(9.0, -9.0);
    let h2 = Point2D::new(9.0, 9.0);
    let h3 = Point2D::new(0.0, 0.0);
    let hull_should_be = vec![h1, h3, h2];
    assert_eq!(hull, hull_should_be);
}

#[test]
// #[ignore]
fn test_chans_algorithm() {
    let mut points: Vec<Point2D> = Vec::new();
    // These points form a triangle, so only the 3 vertices should be in the convex hull.
    for i in 1..10 {
        points.push(Point2D::new(i as f64, i as f64));
        points.push(Point2D::new(i as f64, (-i) as f64));
        points.push(Point2D::new(i as f64, 0.0));
    }
    points.push(Point2D::new(0.0, 0.0));
    let hull = chans_algorithm(&mut points);
    let h1 = Point2D::new(9.0, -9.0);
    let h2 = Point2D::new(9.0, 9.0);
    let h3 = Point2D::new(0.0, 0.0);
    let hull_should_be = vec![h1, h2, h3];
    assert_eq!(hull, hull_should_be);
}

#[test]
fn test_triangle() {
    // triangle has 3 vertices
    let number_of_vertex = 3;

    let vertex_1 = Point2D::new(-1.0000000000000009, -1.7320508075688767);
    let vertex_2 = Point2D::new(2.0, 0.0);
    let vertex_3 = Point2D::new(-0.9999999999999996, 1.7320508075688776);

    let mut input_set_10 = get_input_set(10, number_of_vertex);
    let mut input_set_100 = get_input_set(100, number_of_vertex);
    let mut input_set_10000 = get_input_set(10000, number_of_vertex);
    // let mut input_set_1000000 = get_input_set(1000000, number_of_vertex);
    // let mut input_set_10000000 = get_input_set(10000000, number_of_vertex);

    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_10));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_100));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_10000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_1000000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_10000000));

    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_10));
    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_100));
    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_10000));
    // assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_1000000));
    // assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_10000000));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3], chans_algorithm(&mut input_set_10));
}

#[test]
fn test_triangle1() {
    let vertex_1 = Point2D::new(-1.0000000000000009, -1.7320508075688767); 
    let vertex_2 = Point2D::new(2.0, 0.0); 
    let vertex_3 = Point2D::new(-0.9999999999999996, 1.7320508075688776);   
    let vertex_4 = Point2D::new(0.5601826150542302, -0.7298495252958463); 
    let vertex_5 = Point2D::new(0.6835375132635281, 0.25164023466126156); 
    let vertex_6 = Point2D::new(-0.000313294698257014, 0.0007211333498398356); 
    let vertex_7 = Point2D::new(-0.2643480327658822, -0.10045163625158823); 
    let vertex_8 = Point2D::new(-0.052465944990951086, -0.5964774994916163);
    let mut input_set_8 = vec![vertex_1, vertex_2, vertex_3, vertex_4, vertex_5, vertex_6, vertex_7, vertex_8];
    assert_eq!(vec![vertex_1, vertex_2, vertex_3], graham_scan(&mut input_set_8));

    assert_eq!(vec![vertex_1, vertex_3, vertex_2], jarvis_march(&mut input_set_8));
}
    
#[test]
fn test_rectangle() {
    let number_of_vertex = 4;
    
    let vertex_1 = Point2D::new(-0.00000000000000036739403974420594, -2.0);
    let vertex_2 = Point2D::new(2.0, 0.0);
    let vertex_3 = Point2D::new(0.00000000000000012246467991473532, 2.0);
    let vertex_4 = Point2D::new(-2.0, 0.00000000000000024492935982947064);

    let mut input_set_10 = get_input_set(10, number_of_vertex);
    let mut input_set_100 = get_input_set(100, number_of_vertex);
    let mut input_set_10000 = get_input_set(10000, number_of_vertex);
    // let mut input_set_1000000 = get_input_set(1000000, number_of_vertex);
    // let mut input_set_10000000 = get_input_set(10000000, number_of_vertex);

    assert_eq!(vec![vertex_1, vertex_2, vertex_3, vertex_4], graham_scan(&mut input_set_10));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3, vertex_4], graham_scan(&mut input_set_100));
    assert_eq!(vec![vertex_1, vertex_2, vertex_3, vertex_4], graham_scan(&mut input_set_10000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3, vertex_4], graham_scan(&mut input_set_1000000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3, vertex_4], graham_scan(&mut input_set_10000000));

    assert_eq!(vec![vertex_1, vertex_4, vertex_3, vertex_2], jarvis_march(&mut input_set_10));
    assert_eq!(vec![vertex_1, vertex_4, vertex_3, vertex_2], jarvis_march(&mut input_set_100));
    assert_eq!(vec![vertex_1, vertex_4, vertex_3, vertex_2], jarvis_march(&mut input_set_10000));
    // assert_eq!(vec![vertex_1, vertex_4, vertex_3, vertex_2], jarvis_march(&mut input_set_1000000));
    // assert_eq!(vec![vertex_1, vertex_4, vertex_3, vertex_2], jarvis_march(&mut input_set_10000000));
    // assert_eq!(vec![vertex_1, vertex_2, vertex_3, vertex_4], chans_algorithm(&mut input_set_10));
}


#[test]
fn test_circle() {
    let number_of_vertex = 18;
    
    let vertex_1 = Point2D::new(0.34729635533385816,-1.9696155060244165);
    let vertex_2 = Point2D::new(0.999999999999997,-1.732050807568879);
    let vertex_3 = Point2D::new(1.5320888862379534,-1.2855752193730818);
    let vertex_4 = Point2D::new(1.879385241571815,-0.6840402866513421);
    let vertex_5 = Point2D::new(2.0,0.0);
    let vertex_6 = Point2D::new(1.8793852415718169,0.6840402866513374);
    let vertex_7 = Point2D::new(1.5320888862379562,1.2855752193730785);
    let vertex_8 = Point2D::new(1.0000000000000002,1.7320508075688772);
    let vertex_9 = Point2D::new(0.34729635533386083,1.969615506024416);
    let vertex_10 = Point2D::new(-0.3472963553338606,1.969615506024416);
    let vertex_11 = Point2D::new(-0.9999999999999996,1.7320508075688776);
    let vertex_12 = Point2D::new(-1.5320888862379558,1.285575219373079);
    let vertex_13 = Point2D::new(-1.8793852415718166,0.6840402866513378);
    let vertex_14 = Point2D::new(-2.0,0.00000000000000024492935982947064);
    let vertex_15 = Point2D::new(-1.8793852415718169,-0.6840402866513373);
    let vertex_16 = Point2D::new(-1.5320888862379562,-1.2855752193730785);
    let vertex_17 = Point2D::new(-1.0000000000000009,-1.7320508075688767);
    let vertex_18 = Point2D::new(-0.34729635533386244,-1.9696155060244158);

    let mut input_set_100 = get_input_set(100, number_of_vertex);
    let mut input_set_10000 = get_input_set(10000, number_of_vertex);
    // let mut input_set_1000000 = get_input_set(1000000, number_of_vertex);
    // let mut input_set_10000000 = get_input_set(10000000, number_of_vertex);

    let hull_should_be = vec![vertex_1, vertex_2, vertex_3, vertex_4, 
                            vertex_5, vertex_6, vertex_7, vertex_8, vertex_9, vertex_10, 
                            vertex_11, vertex_12, vertex_13, vertex_14, vertex_15, vertex_16, 
                            vertex_17, vertex_18];

    assert_eq!(hull_should_be, graham_scan(&mut input_set_100));
    assert_eq!(hull_should_be, graham_scan(&mut input_set_10000));
    // assert_eq!(hull_should_be, graham_scan(&mut input_set_1000000));
    // assert_eq!(hull_should_be, graham_scan(&mut input_set_10000000));

    let hull_should_be = vec![vertex_1,vertex_18,vertex_17,
                            vertex_16,vertex_15,vertex_14,
                            vertex_13,vertex_12,vertex_11,vertex_10
                            ,vertex_9,vertex_8,vertex_7,
                            vertex_6,vertex_5,vertex_4,
                            vertex_3,vertex_2];

    assert_eq!(hull_should_be, jarvis_march(&mut input_set_100));
    assert_eq!(hull_should_be, jarvis_march(&mut input_set_10000));
    // assert_eq!(hull_should_be, jarvis_march(&mut input_set_1000000));
    // assert_eq!(hull_should_be, jarvis_march(&mut input_set_10000000));
    // assert_eq!(hull_should_be, chans_algorithm(&mut input_set_100));
}