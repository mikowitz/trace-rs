use itertools::Itertools;
use rand::*;
use trace_rs::{
    camera::Camera,
    color::Color,
    material::Material,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    let mat1 = Material::Dielectric(1.5);
    let mat2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    let mat3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.);
    let mut world = vec![
        Sphere::new(Point3::new(0., -1000., -1.), 1000., ground_material),
        Sphere::new(Point3::new(0., 1., 0.), 1., mat1),
        Sphere::new(Point3::new(-4., 1., 0.), 1., mat2),
        Sphere::new(Point3::new(4., 1., 0.), 1., mat3),
    ];

    let mut rng = rand::thread_rng();

    (-11..11)
        .cartesian_product(-11..11)
        .map(|(a, b)| {
            let choose_mat = rng.gen::<f32>();
            let center = Point3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let mat = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Material::Lambertian(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Material::Metal(albedo, fuzz)
                } else {
                    Material::Dielectric(1.5)
                };
                world.push(Sphere::new(center, 0.2, mat));
            }
        })
        .for_each(drop);

    let mut camera = Camera::default();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookto = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera.render(&world);
}
