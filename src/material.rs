use crate::{hittable::HitRecord, ray::Ray, vector::random_unit_vector};
use glam::Vec3;
use rand::Rng;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dieletric(f32),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian(albedo) => {
                let mut scatter_direction = hit_rec.normal + random_unit_vector();

                if scatter_direction.abs().cmplt(Vec3::splat(1e-8)).all() {
                    scatter_direction = hit_rec.normal
                };
                let scattered = Ray {
                    origin: hit_rec.p,
                    direction: scatter_direction,
                };
                Some(ScatterRecord {
                    attenuation: *albedo,
                    scattered,
                })
            }
            Self::Metal(albedo, fuzz) => {
                let mut reflected = ray.direction.reflect(hit_rec.normal);
                reflected = reflected.normalize() + (fuzz * random_unit_vector());
                let scattered = Ray {
                    origin: hit_rec.p,
                    direction: reflected,
                };
                if scattered.direction.dot(hit_rec.normal) > 0. {
                    Some(ScatterRecord {
                        attenuation: *albedo,
                        scattered,
                    })
                } else {
                    None
                }
            }
            Self::Dieletric(refraction_index) => {
                let ri = if hit_rec.front_face {
                    1. / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = ray.direction.normalize();
                let cos_theta = -unit_direction.dot(hit_rec.normal).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let mut rng = rand::thread_rng();
                let direction =
                    if ri * sin_theta > 1. || reflectance(cos_theta, ri) > rng.gen::<f32>() {
                        unit_direction.reflect(hit_rec.normal)
                    } else {
                        unit_direction.refract(hit_rec.normal, ri)
                    };

                let scattered = Ray {
                    origin: hit_rec.p,
                    direction,
                };

                Some(ScatterRecord {
                    attenuation: Vec3::ONE,
                    scattered,
                })
            }
        }
    }
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
