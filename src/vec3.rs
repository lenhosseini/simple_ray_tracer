use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use image::Rgb;
use rand::{
    distributions::{Distribution, Standard},
    rngs::ThreadRng,
    Rng,
};
use std::{fmt::Display, ops};

#[derive(
    Debug, Clone, Copy, Add, AddAssign, Sub, SubAssign, MulAssign, Mul, Div, DivAssign, Neg,
)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.);
    pub const ONE: Self = Self::splat(1.);
    pub const NEG_ONE: Self = Self::splat(-1.);
    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);
    pub const NEG_X: Self = Self::new(-1., 0., 0.);
    pub const NEG_Y: Self = Self::new(0., -1., 0.);
    pub const NEG_Z: Self = Self::new(0., 0., -1.);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(value: f64) -> Self {
        Self::new(value, value, value)
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub const fn z(&self) -> f64 {
        self.z
    }

    pub fn r(&self) -> u8 {
        Self::as_u8(u8::MAX as f64 * self.x)
    }

    pub fn g(&self) -> u8 {
        Self::as_u8(u8::MAX as f64 * self.y)
    }

    pub fn b(&self) -> u8 {
        Self::as_u8(u8::MAX as f64 * self.z)
    }

    fn as_u8(value: f64) -> u8 {
        value.clamp(u8::MIN as f64, u8::MAX as f64) as u8
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        Rgb([self.r(), self.g(), self.b()])
    }

    pub fn gen(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }

    pub fn gen_range(rng: &mut ThreadRng, min: f64, max: f64) -> Self {
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        let min = -1.;
        let max = 1.;

        loop {
            let p = Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), 0.);

            if p.length_squared() < 1. {
                break p;
            }
        }
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let p = Self::gen_range(rng, -1., 1.);

            if p.length_squared() < 1. {
                break p;
            }
        }
    }

    pub fn random_unit(rng: &mut ThreadRng) -> Vec3 {
        Self::random_in_unit_sphere(rng).unit()
    }

    pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit(rng);

        match on_unit_sphere.dot(normal) {
            dot if dot > 0. => on_unit_sphere,
            _ => -on_unit_sphere,
        }
    }

    pub fn reflect(&self, rhs: Vec3) -> Vec3 {
        *self - 2. * self.dot(rhs) * rhs
    }

    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -self.dot(n).min(1.);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -(1. - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        (1. / self) * rhs
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.r(), self.g(), self.b())
    }
}
