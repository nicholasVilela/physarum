use super::*;


fn calculate_pixel_list(radius: i32) -> Vec<(i32, i32)> {
    let mut pixel_list = vec![(0,0)];

    if radius > 0 {
        for y in -radius..radius + 1 {
            for x in -radius..radius + 1{
                let t = (x, y);
                if t == (0,0) { continue; }

                pixel_list.push(t);
            }
        }
    }

    return pixel_list;
}

#[test]
fn pixel_radius_0() {
    let radius = 0;

    let pixel_list = calculate_pixel_list(radius);
    let target_pixel_list = vec![(0, 0)];

    assert_eq!(pixel_list, target_pixel_list);
}

#[test]
fn pixel_radius_1() {
    let radius = 1;

    let pixel_list = calculate_pixel_list(radius);
    let target_pixel_list = vec![
        (0,0), (-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1),
    ];

    assert_eq!(pixel_list, target_pixel_list);
}

#[test]
fn pixel_radius_2() {
    let radius = 2;

    let pixel_list = calculate_pixel_list(radius);
    let target_pixel_list = vec![
        (0,0), (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2), (-2,-1), (-1,-1), (0,-1), (1,-1), (2,-1), (-2,0), (-1,0), (1,0), (2,0), (-2,1), (-1,1), (0,1), (1,1), (2,1), (-2,2), (-1,2), (0,2), (1,2), (2,2)
    ];

    assert_eq!(pixel_list, target_pixel_list);
}