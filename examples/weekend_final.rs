use glam::Vec3;
use itertools::Itertools;
use rand::Rng;
use trace_rs::{
    camera::Camera,
    hittable_list::HittableList,
    material::Material,
    sphere::Sphere,
    vector::{random_vec3, random_vec3_in},
};

fn main() {
    let mut world: HittableList<Sphere> = HittableList::new();

    let ground_material = Material::Lambertian(Vec3::splat(0.5));

    world.add(Sphere {
        center: Vec3::NEG_Y * 1000.,
        radius: 1000.,
        material: ground_material,
    });

    let mut rng = rand::thread_rng();
    (-11..11).cartesian_product(-11..11).for_each(|(a, b)| {
        let choose_mat = rng.gen::<f32>();
        let center = Vec3::new(
            a as f32 + 0.9 * rng.gen::<f32>(),
            0.2,
            b as f32 + 0.9 * rng.gen::<f32>(),
        );

        if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
            let mut material = Material::Lambertian(random_vec3() * random_vec3());

            if (0.8..0.95).contains(&choose_mat) {
                material = Material::Metal(random_vec3_in(0.5..1.0), rng.gen_range(0.0..0.5));
            } else if choose_mat > 0.95 {
                material = Material::Dieletric(1.5);
            };

            world.add(Sphere {
                center,
                radius: 0.2,
                material,
            });
        }
    });

    let mat1 = Material::Dieletric(1.5);
    world.add(Sphere {
        center: Vec3::Y,
        radius: 1.,
        material: mat1,
    });

    let mat2 = Material::Lambertian(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere {
        center: Vec3::Y + Vec3::NEG_X * 4.,
        radius: 1.,
        material: mat2,
    });

    let mat3 = Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.);
    world.add(Sphere {
        center: Vec3::Y + Vec3::X * 4.,
        radius: 1.,
        material: mat3,
    });

    let mut cam = Camera::new();
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Vec3::new(13., 2., 3.);
    cam.lookat = Vec3::ZERO;
    cam.vup = Vec3::Y;

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
