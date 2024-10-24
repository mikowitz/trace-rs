use itertools::Itertools;
use rand::*;
use trace_rs::{
    bvh_node::BvhNode,
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::Material,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    let mat1 = Material::Dielectric(1.5);
    let mat2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    let mat3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.);
    let mut world: HittableList<Sphere> = HittableList::new();
    world.add(Sphere::new(
        Point3::new(0., -1000., -1.),
        1000.,
        ground_material,
    ));
    world.add(Sphere::new(Point3::new(0., 1., 0.), 1., mat1));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1., mat2));
    world.add(Sphere::new(Point3::new(4., 1., 0.), 1., mat3));

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
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let mat = Material::Lambertian(albedo);
                    if rng.gen::<f32>() > 0.5 {
                        world.add(Sphere::moving(
                            center,
                            center + Vec3::new(0., rng.gen_range(0.0..0.5), 0.),
                            0.2,
                            mat,
                        ));
                    } else {
                        world.add(Sphere::new(center, 0.2, mat));
                    }
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let mat = Material::Metal(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, mat));
                } else {
                    let mat = Material::Dielectric(1.5);
                    world.add(Sphere::new(center, 0.2, mat));
                };
            }
        })
        .for_each(drop);

    let mut camera = Camera::default();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 1200;
    camera.samples_per_pixel = 100;
    camera.max_depth = 10;

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookto = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    let bvh = BvhNode::new(world.clone());
    camera.render(&bvh);
}
