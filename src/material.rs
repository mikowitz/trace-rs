use glam::Vec3;

use crate::{hittable::HitRecord, ray::Ray, vector::random_unit_vector};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
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
        }
    }
}
