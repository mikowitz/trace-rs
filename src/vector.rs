use glam::Vec3;
use rand::Rng;
use std::ops::Range;

pub fn random_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn random_vec3_in(interval: Range<f32>) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(interval.start..interval.end),
        rng.gen_range(interval.start..interval.end),
        rng.gen_range(interval.start..interval.end),
    )
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_vec3_in(-1.0..1.0);
        if (1e-160_f32..1.0).contains(&p.length_squared()) {
            break p.normalize();
        }
    }
}

pub fn random_vec3_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    }
    -on_unit_sphere
}
