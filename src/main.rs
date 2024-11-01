use glam::Vec3;
use trace_rs::{camera::Camera, hittable_list::HittableList, sphere::Sphere};

fn main() {
    let mut world: HittableList<Sphere> = HittableList::new();
    world.add(Sphere {
        center: -Vec3::Z,
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
    });

    let mut cam = Camera::new();
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 800;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;

    cam.render(&world);
}
