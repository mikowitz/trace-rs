use trace_rs::{
    camera::Camera,
    color::Color,
    material::Material,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let ground_material = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
    let center_material = Material::Lambertian(Color::new(0.1, 0.2, 0.5));
    let left_material = Material::Dielectric(1.5);
    let bubble_material = Material::Dielectric(1.0 / 1.5);
    let right_material = Material::Metal(Color::new(0.8, 0.6, 0.2), 1.0);
    let world = vec![
        Sphere::new(Point3::new(0., -100.5, -1.), 100., ground_material),
        Sphere::new(Point3::new(0., 0., -1.2), 0.5, center_material),
        Sphere::new(Point3::new(-1., 0., -1.), 0.5, left_material),
        Sphere::new(Point3::new(-1., 0., -1.), 0.4, bubble_material),
        Sphere::new(Point3::new(1., 0., -1.), 0.5, right_material),
    ];

    let mut camera = Camera::default();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.;
    camera.lookfrom = Point3::new(-2., 2., 1.);
    camera.lookto = Point3::new(0., 0., -1.);
    camera.vup = Vec3::new(0., 1., 0.);
    camera.defocus_angle = 10.;
    camera.focus_dist = 3.4;

    camera.render(&world);
}
