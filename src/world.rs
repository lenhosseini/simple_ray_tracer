use crate::{Color, Hittable, HittableList, HittableObj, Material, Point3, Sphere, Vec3};
use rand::{thread_rng, Rng};

pub struct World(HittableList);

impl World {
    pub const fn new(hittables: HittableList) -> Self {
        Self(hittables)
    }

    pub fn hittables(self) -> HittableList {
        self.0
    }

    pub fn scene() -> Self {
        let mut hittables: Vec<Box<dyn Hittable>> = vec![];

        let mat_ground = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
        hittables.push(Box::new(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            mat_ground,
        )));

        let mut rng = thread_rng();

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.gen::<f64>();
                let center = Point3::new(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );

                if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                    let mat_spehere = match choose_mat {
                        x if x < 0.8 => {
                            let albedo: Color = Vec3::gen(&mut rng) * Vec3::gen(&mut rng);
                            Material::Lambertian(albedo)
                        }
                        x if x < 0.95 => {
                            let albedo: Color = Vec3::gen_range(&mut rng, 0.5, 1.);
                            let fuzz = rng.gen::<f64>();
                            Material::Metal { albedo, fuzz }
                        }
                        _ => Material::Dialectric(1.5),
                    };

                    hittables.push(Box::new(Sphere::new(center, 0.2, mat_spehere)));
                }
            }
        }

        let mat_2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
        hittables.push(Box::new(Sphere::new(Point3::new(-4., 1., 0.), 1., mat_2)));

        let mat_1 = Material::Dialectric(1.5);
        hittables.push(Box::new(Sphere::new(Point3::Y, 1., mat_1)));

        let mat_3 = Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        };
        hittables.push(Box::new(Sphere::new(Point3::new(4., 1., 0.), 1., mat_3)));

        Self::new(hittables)
    }
}

impl IntoIterator for World {
    type Item = HittableObj;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Hittable for World {
    fn hit(&self, ray: &crate::Ray, ray_t: &crate::Interval) -> crate::HitResult {
        self.0.hit(ray, ray_t)
    }
}
