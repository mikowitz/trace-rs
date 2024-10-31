use glam::Vec3;

pub fn to_ppm(color: Vec3) -> String {
    let c = color * 255.999;

    format!("{} {} {}", c.x as u8, c.y as u8, c.z as u8)
}
