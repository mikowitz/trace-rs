use trace_rs::{camera::Camera, sphere::Sphere, vec3::Point3};

fn main() {
    let mut world = vec![];
    world.push(Sphere::new(Point3::new(0., 0., -1.), 0.5));
    world.push(Sphere::new(Point3::new(0., -100.5, -1.), 100.));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;

    camera.render(&world);
}
