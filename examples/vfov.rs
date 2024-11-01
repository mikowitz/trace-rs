use std::f32::consts::PI;

use glam::Vec3;
use trace_rs::{camera::Camera, hittable_list::HittableList, material::Material, sphere::Sphere};

fn main() {
    let r = (PI / 4.).cos();

    let mut world: HittableList<Sphere> = HittableList::new();
    let left_material = Material::Lambertian(Vec3::Z);
    let right_material = Material::Lambertian(Vec3::X);
    world.add(Sphere {
        center: Vec3::new(-r, 0., -1.),
        radius: r,
        material: left_material,
    });
    world.add(Sphere {
        center: Vec3::new(r, 0., -1.),
        radius: r,
        material: right_material,
    });

    let mut cam = Camera::new();
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 800;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;
    cam.vfov = 90.;

    cam.render(&world);
}
