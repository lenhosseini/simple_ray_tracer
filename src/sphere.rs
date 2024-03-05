use crate::{HitRecord, HitResult, Hittable, Interval, Material, Point3, Ray};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> HitResult {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        match discriminant < 0. {
            true => return None,
            false => (),
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        match !ray_t.surrounds(root) {
            true => {
                root = (-half_b + sqrtd) / a;
                match !ray_t.surrounds(root) {
                    true => return None,
                    false => (),
                }
            }
            false => (),
        };

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, t, ray, outward_normal, self.material))
    }
}
