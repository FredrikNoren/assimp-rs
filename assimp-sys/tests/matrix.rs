extern crate assimp_sys;
extern crate cgmath;
use assimp_sys::*;

#[test]
fn test_matrix3_conversion() {
    let x = cgmath::Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let y = AiMatrix3x3::from_cgmath_matrix(&x);

    let z = AiMatrix3x3 {
        a1: 1.0, a2: 4.0, a3: 7.0,
        b1: 2.0, b2: 5.0, b3: 8.0,
        c1: 3.0, c2: 6.0, c3: 9.0,
    };

    assert_eq!(y, z)
}

#[test]
fn test_matrix4_conversion() {
    let x = cgmath::Matrix4::new(
        1.0,   2.0,  3.0,  4.0,
        5.0,   6.0,  7.0,  8.0,
        9.0,  10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0);
    let y = AiMatrix4x4::from_cgmath_matrix(&x);

    let z = AiMatrix4x4 {
        a1: 1.0, a2: 5.0, a3:  9.0, a4: 13.0,
        b1: 2.0, b2: 6.0, b3: 10.0, b4: 14.0,
        c1: 3.0, c2: 7.0, c3: 11.0, c4: 15.0,
        d1: 4.0, d2: 8.0, d3: 12.0, d4: 16.0
    };

    assert_eq!(y, z)
}
