extern crate assimp;
extern crate cgmath;
extern crate libc;
use assimp::math::*;
use libc::c_float;

#[test]
fn test_matrix3_conversion() {
    use cgmath::Matrix3;
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
    use cgmath::Matrix4;
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
    use cgmath::Quaternion as CgQuaternion;

    // cgmath quaternion
    let q1 = CgQuaternion::new(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion::from(q1);
    let q3: CgQuaternion<c_float> = q2.into();
    assert_eq!(q1, q3);
}

#[test]
fn test_vector2_conversion() {
    use cgmath::{Point2, Vector2};

    // cgmath vector
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2D::from(v1);
    let v3: Vector2<f32> = v2.into();
    assert_eq!(v1, v3);

    // cgmath point
    let v1 = Point2::new(1.0, 2.0);
    let v2 = Vector2D::from(v1);
    let v3: Point2<f32> = v2.into();
    assert_eq!(v1, v3);

    // fixed array type
    let v1 = [1.0, 2.0];
    let v2 = Vector2D::from(v1);
    let v3: [f32; 2] = v2.into();
    assert_eq!(v1, v3);
}

#[test]
fn test_vector3_conversion() {
    use cgmath::{Point3, Vector3};

    // cgmath vector
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::from(v1);
    let v3: Vector3<f32> = v2.into();
    assert_eq!(v1, v3);

    // cgmath point
    let v1 = Point3::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::from(v1);
    let v3: Point3<f32> = v2.into();
    assert_eq!(v1, v3);

    // fixed array type
    let v1 = [1.0, 2.0, 3.0];
    let v2 = Vector3D::from(v1);
    let v3: [f32; 3] = v2.into();
    assert_eq!(v1, v3);
}

#[test]
fn test_color3_conversion() {
    use cgmath::Vector3;

    // cgmath vector
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Color3D::from(v1);
    let v3: Vector3<f32> = v2.into();
    assert_eq!(v1, v3);

    // fixed array type
    let v1 = [1.0, 2.0, 3.0];
    let v2 = Color3D::from(v1);
    let v3: [f32; 3] = v2.into();
    assert_eq!(v1, v3);
}

#[test]
fn test_color4_conversion() {
    use cgmath::Vector4;

    // cgmath vector
    let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let v2 = Color4D::from(v1);
    let v3: Vector4<f32> = v2.into();
    assert_eq!(v1, v3);

    // fixed array type
    let v1 = [1.0, 2.0, 3.0, 4.0];
    let v2 = Color4D::from(v1);
    let v3: [f32; 4] = v2.into();
    assert_eq!(v1, v3);
}
