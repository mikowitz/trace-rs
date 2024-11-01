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

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.);
        if p.length_squared() < 1. {
            break p;
        }
    }
}
