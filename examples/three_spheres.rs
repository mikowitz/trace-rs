use glam::Vec3;
use trace_rs::{camera::Camera, hittable_list::HittableList, material::Material, sphere::Sphere};

fn main() {
    let mut world: HittableList<Sphere> = HittableList::new();
    let ground_material = Material::Lambertian(Vec3::new(0.8, 0.8, 0.0));
    let center_material = Material::Lambertian(Vec3::new(0.1, 0.2, 0.5));
    let left_material = Material::Dieletric(1.5);
    let bubble_material = Material::Dieletric(1.0 / 1.5);
    let right_material = Material::Metal(Vec3::new(0.8, 0.6, 0.2), 1.0);
    world.add(Sphere {
        center: Vec3::NEG_Z * 1.2,
        radius: 0.5,
        material: center_material,
    });
    world.add(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
        material: ground_material,
    });
    world.add(Sphere {
        center: Vec3::NEG_Z + Vec3::NEG_X,
        radius: 0.5,
        material: left_material,
    });
    world.add(Sphere {
        center: Vec3::NEG_Z + Vec3::NEG_X,
        radius: 0.4,
        material: bubble_material,
    });
    world.add(Sphere {
        center: Vec3::NEG_Z + Vec3::X,
        radius: 0.5,
        material: right_material,
    });

    let mut cam = Camera::new();
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 800;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;

    cam.vfov = 20.;
    cam.lookfrom = Vec3::new(-2., 2., 1.);
    cam.lookat = Vec3::NEG_Z;
    cam.vup = Vec3::Y;

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
