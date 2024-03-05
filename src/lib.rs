mod camera;
mod hit;
mod image;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod world;

pub use camera::*;
pub use hit::*;
pub use image::*;
pub use interval::*;
pub use material::*;
pub use ray::*;
pub use sphere::*;
pub use utils::*;
pub use vec3::*;
pub use world::*;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub type Result<T> = anyhow::Result<T>;
