use rand::{rngs::ThreadRng, Rng};

use crate::{Color, HitRecord, Ray, Vec3};

pub struct Scatter {
    pub attenuation: Color,
    pub scatter: Ray,
}

impl Scatter {
    pub const fn new(attenuation: Color, scatter: Ray) -> Self {
        Self {
            attenuation,
            scatter,
        }
    }
}

pub trait Reflect {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal { albedo: Color, fuzz: f64 },
    Dialectric(f64),
}

impl Reflect for Material {
    fn scatter(&self, rng: &mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_dir = rec.normal + Vec3::random_unit(rng);

                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal
                }

                let scatter = Ray::new(rec.p, scatter_dir);

                Some(Scatter::new(*albedo, scatter))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = r_in.direction().unit().reflect(rec.normal);
                let scatter = Ray::new(rec.p, reflected + *fuzz * Vec3::random_unit(rng));

                match scatter.direction().dot(rec.normal) > 0. {
                    true => Some(Scatter::new(*albedo, scatter)),
                    false => None,
                }
            }
            Material::Dialectric(ir) => {
                let attenuation = Color::ONE;
                let refraction_ratio = if rec.font_face { 1. / ir } else { *ir };

                let unit_dir = r_in.direction().unit();
                let cos_theta = -unit_dir.dot(rec.normal).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.;

                let dir = if cannot_refract
                    || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
                {
                    unit_dir.reflect(rec.normal)
                } else {
                    unit_dir.refract(rec.normal, refraction_ratio)
                };

                let scatter = Ray::new(rec.p, dir);
                Some(Scatter::new(attenuation, scatter))
            }
        }
    }
}

impl Material {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}
