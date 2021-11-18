use super::*;
use na::Vector3;
use nalgebra::*;

fn angle_between_vectors<T>(v1: &Vector3<T>, v2: &Vector3<T>) -> f64
where
    T: ComplexField<RealField = f64> + Into<f64>,
{
    //Formula =  a · b = |a| × |b| × cos(θ)

    let dot_product: f64 = v1.dot(v2).into();
    let cos_of_angle = dot_product / (v1.norm() * v2.norm());
    cos_of_angle.acos() // in radians
}

#[cfg(test)]
mod tests {
    use super::*;
    use all_asserts::*;

    #[test]
    fn angle_between_vectors_test() {
        assert_near!(
            angle_between_vectors(&Vector3::new(0.0, 1.0, 0.0), &Vector3::new(0.0, 1.0, 1.0))
                .to_degrees(),
            45.0,
            0.0001
        )
    }
    #[test]
    fn angle_between_vectors2_test() {
        assert_near!(
            angle_between_vectors(&Vector3::new(-1.0, 1.0, 0.0), &Vector3::new(1.0, 1.0, 0.0))
                .to_degrees(),
            90.0,
            0.0001
        )
    }
    #[test]
    fn angle_between_same_vectors_test() {
        assert_near!(
            angle_between_vectors(&Vector3::new(0.0, 1.0, 0.0), &Vector3::new(0.0, 1.0, 0.0))
                .to_degrees(),
            0.0,
            0.0001
        )
    }

    #[test]
    fn angle_between_opposite_vectors_test() {
        assert_near!(
            angle_between_vectors(&Vector3::new(0.0, 0.0, -1.0), &Vector3::new(0.0, 0.0, 1.0))
                .to_degrees(),
            180.0,
            0.0001
        )
    }

    #[test]
    fn angle_from_camera_to_face_test() {
        let camera_angle = Vector3::new(0.0, 0.0, 1.0);
        let face_normal = Vector3::new(1.0, 0.0, 1.0);
        let radians = angle_between_vectors(&camera_angle, &face_normal);
        assert_near!(radians.to_degrees(), 45.0, 0.0001);
    }
}
