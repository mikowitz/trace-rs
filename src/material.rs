use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f32),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian(albedo) => lambertian_scatter(hit_rec, *albedo),
            Self::Metal(albedo, fuzz) => metal_scatter(ray, hit_rec, *albedo, *fuzz),
        }
    }
}

fn lambertian_scatter(hit_rec: &HitRecord, albedo: Color) -> Option<ScatterRecord> {
    let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
    if scatter_direction.is_near_zero() {
        scatter_direction = hit_rec.normal;
    }
    Some(ScatterRecord {
        attenuation: albedo,
        scattered: Ray::new(hit_rec.p, scatter_direction),
    })
}

fn metal_scatter(
    ray: &Ray,
    hit_rec: &HitRecord,
    albedo: Color,
    fuzz: f32,
) -> Option<ScatterRecord> {
    let mut reflected = ray.direction.reflect(&hit_rec.normal);
    reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * fuzz);
    let scattered = Ray::new(hit_rec.p, reflected);

    if scattered.direction.dot(&hit_rec.normal) <= 0. {
        return None;
    }

    Some(ScatterRecord {
        scattered,
        attenuation: albedo,
    })
}
