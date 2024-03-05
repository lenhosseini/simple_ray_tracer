use rand::rngs::ThreadRng;

use crate::{Color, Hittable, Interval, Point3, Reflect, Vec3, World, INFINITY};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub const fn origin(&self) -> Point3 {
        self.origin
    }

    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, rng: &mut ThreadRng, depth: u16, world: &World) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        };

        if let Some(rec) = world.hit(self, &Interval::new(0.001, INFINITY)) {
            match rec.material.scatter(rng, self, &rec) {
                Some(scatter) => {
                    return scatter.attenuation * scatter.scatter.color(rng, depth - 1, world)
                }
                None => return Color::ZERO,
            };
        };

        let unit_dir = self.direction.unit();
        let a = 0.5 * (unit_dir.y() + 1.);
        (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
    }
}
