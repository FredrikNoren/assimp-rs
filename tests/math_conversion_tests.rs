extern crate assimp;
extern crate cgmath;
extern crate libc;
use assimp::math::*;
use cgmath::{Matrix3, Matrix4, Point3, Vector3, Vector4};
use cgmath::Quaternion as CgQuaternion;
use libc::c_float;

#[test]
fn test_matrix3_conversion() {
    let m1 = Matrix3::new(1.0, 2.0, 3.0,
                          4.0, 5.0, 6.0,
                          7.0, 8.0, 9.0);
    let m2 = Matrix3x3::new(1.0, 4.0, 7.0,
                            2.0, 5.0, 8.0,
                            3.0, 6.0, 9.0);

    let m3 = Matrix3x3::from(m1);
    let m4: Matrix3<c_float> = m2.into();

    assert_eq!(m1, m4);
    assert_eq!(m2, m3);
}

#[test]
fn test_matrix4_conversion() {
    let m1 = Matrix4::new(1.0, 2.0, 3.0, 4.0,
                          5.0, 6.0, 7.0, 8.0,
                          9.0, 10.0, 11.0, 12.0,
                          13.0, 14.0, 15.0, 16.0);
    let m2 = Matrix4x4::new(1.0, 5.0, 9.0, 13.0,
                            2.0, 6.0, 10.0, 14.0,
                            3.0, 7.0, 11.0, 15.0,
                            4.0, 8.0, 12.0, 16.0);

    let m3 = Matrix4x4::from(m1);
    let m4: Matrix4<c_float> = m2.into();

    assert_eq!(m1, m4);
    assert_eq!(m2, m3);
}

#[test]
fn test_quaternion_conversion() {
    let q1 = CgQuaternion::new(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let q3 = Quaternion::from(q1);
    let q4: CgQuaternion<c_float> = q2.into();

    assert_eq!(q1, q4);
    assert_eq!(q2, q3);
}

#[test]
fn test_vector3_conversion() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(1.0, 2.0, 3.0);
    let v3 = Vector3D::from(v1);
    let v4: Vector3<c_float> = v2.into();

    assert_eq!(v1, v4);
    assert_eq!(v2, v3);
}

#[test]
fn test_point3_conversion() {
    let v1 = Point3::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(1.0, 2.0, 3.0);
    let v3 = Vector3D::from(v1);
    let v4: Point3<c_float> = v2.into();

    assert_eq!(v1, v4);
    assert_eq!(v2, v3);
}

#[test]
fn test_color3_conversion() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Color3D::new(1.0, 2.0, 3.0);
    let v3 = Color3D::from(v1);
    let v4: Vector3<c_float> = v2.into();

    assert_eq!(v1, v4);
    assert_eq!(v2, v3);
}

#[test]
fn test_color4_conversion() {
    let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let v2 = Color4D::new(1.0, 2.0, 3.0, 4.0);
    let v3 = Color4D::from(v1);
    let v4: Vector4<c_float> = v2.into();

    assert_eq!(v1, v4);
    assert_eq!(v2, v3);
}
