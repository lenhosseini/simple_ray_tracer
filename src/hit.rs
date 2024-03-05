use crate::{Interval, Material, Point3, Ray, Vec3};

pub type HitResult = Option<HitRecord>;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub font_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: Vec3, material: Material) -> Self {
        let font_face = ray.direction().dot(outward_normal) < 0.;

        let normal = if font_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            font_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> HitResult;
}

pub type HittableObj = Box<dyn Hittable>;

pub type HittableList = Vec<HittableObj>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> HitResult {
        let mut temp_rec: HitResult = None;

        for hittable in self.iter() {
            match hittable.hit(ray, ray_t) {
                Some(rec) => temp_rec = Some(rec),
                None => (),
            }
        }

        temp_rec
    }
}
