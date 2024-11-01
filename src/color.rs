use glam::Vec3;

pub fn to_ppm(color: Vec3) -> String {
    let c = color
        .map(linear_to_gamma)
        .clamp(Vec3::splat(0.000), Vec3::splat(0.999))
        * 256.0;

    format!("{} {} {}", c.x as u8, c.y as u8, c.z as u8)
}

pub fn linear_to_gamma(linear: f32) -> f32 {
    if linear <= 0. {
        return 0.;
    }
    linear.sqrt()
}
