use glam::Vec3;
use itertools::Itertools;
use rand::Rng;
use trace_rs::{
    bvh_node::BvhNode,
    camera::Camera,
    hittable_list::HittableList,
    material::Material,
    sphere::Sphere,
    vector::{random_vec3, random_vec3_in},
};

fn main() {
    let mut world: HittableList<Sphere> = HittableList::new();

    let ground_material = Material::Lambertian(Vec3::splat(0.5));

    world.add(Sphere::new(Vec3::NEG_Y * 1000., 1000., ground_material));

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

            if rng.gen::<f32>() > 0.75 && choose_mat < 0.8 {
                let center2 = center + Vec3::ZERO.with_y(rng.gen_range(0.0..0.5));
                world.add(Sphere::moving(center, center2, 0.2, material));
            } else {
                world.add(Sphere::new(center, 0.2, material));
            }
        }
    });

    let mat1 = Material::Dieletric(1.5);
    world.add(Sphere::new(Vec3::Y, 1., mat1));

    let mat2 = Material::Lambertian(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Vec3::Y + Vec3::NEG_X * 4., 1., mat2));

    let mat3 = Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.);
    world.add(Sphere::new(Vec3::Y + Vec3::X * 4., 1., mat3));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 1200;
    cam.samples_per_pixel = 50;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Vec3::new(13., 2., 3.);
    cam.lookat = Vec3::ZERO;
    cam.vup = Vec3::Y;

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    println!("{}", world.objects.len());
    let bvh = BvhNode::new(world.clone());
    cam.render(&bvh);
}
