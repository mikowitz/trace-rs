use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};
use rand::*;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f32),
    Dielectric(f32),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian(albedo) => lambertian_scatter(hit_rec, *albedo),
            Self::Metal(albedo, fuzz) => metal_scatter(ray, hit_rec, *albedo, *fuzz),
            Self::Dielectric(refraction_index) => {
                dielectric_scatter(ray, hit_rec, *refraction_index)
            }
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

fn dielectric_scatter(
    ray: &Ray,
    hit_rec: &HitRecord,
    refraction_index: f32,
) -> Option<ScatterRecord> {
    let ri = if hit_rec.front_face {
        1.0 / refraction_index
    } else {
        refraction_index
    };

    let unit_direction = ray.direction.unit_vector();
    let cos_θ = -unit_direction.dot(&hit_rec.normal).min(1.0);
    let sin_θ = (1.0 - cos_θ * cos_θ).sqrt();

    let cannot_refract = ri * sin_θ > 1.0;

    let mut rng = rand::thread_rng();
    let direction = if cannot_refract || reflectance(cos_θ, ri) > rng.gen::<f32>() {
        unit_direction.reflect(&hit_rec.normal)
    } else {
        unit_direction.refract(&hit_rec.normal, ri)
    };

    Some(ScatterRecord {
        attenuation: Color::white(),
        scattered: Ray::new(hit_rec.p, direction),
    })
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
