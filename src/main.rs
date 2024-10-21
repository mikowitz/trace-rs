use trace_rs::{camera::Camera, sphere::Sphere, vec3::Point3};

fn main() {
    let world = vec![
        Sphere::new(Point3::new(0., 0., -1.), 0.5),
        Sphere::new(Point3::new(0., -100.5, -1.), 100.),
    ];

    let mut camera = Camera::default();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
