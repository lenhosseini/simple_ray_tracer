use crate::{progress_bar, write_color, Color, Image, Point3, Ray, Result, Vec3, World};

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Camera {
    image_width: i64,
    samples_per_pixel: i64,
    image_height: i64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: u16,
    lookfrom: Point3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    pb: ProgressBar,
}

impl Camera {
    pub fn new(
        img: Image,
        samples_per_pixel: i64,
        max_depth: u16,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        Self::initialize(
            img,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        )
    }

    fn initialize(
        img: Image,
        samples_per_pixel: i64,
        max_depth: u16,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let center = lookfrom;

        // Camera

        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (img.width() as f64 / img.height() as f64);

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_width * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / img.width() as f64;
        let pixel_delta_v = viewport_v / img.height() as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pb = progress_bar(img.width().into(), img.height().into());

        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            lookfrom,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            pb,
            image_width: img.width().into(),
            image_height: img.height().into(),
        }
    }

    pub fn render(&self, world: World) -> Result<RgbImage> {
        let mut img_buffer: RgbImage =
            ImageBuffer::new(self.image_width.try_into()?, self.image_height.try_into()?);

        let mut rng = thread_rng();

        for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
            let mut pixel_color = Color::ZERO;

            for _ in 0..self.samples_per_pixel {
                let ray = Self::get_ray(self, &mut rng, x.into(), y.into());
                pixel_color += ray.color(&mut rng, self.max_depth, &world);
            }

            self.pb.inc(1);

            *pixel = write_color(pixel_color, self.samples_per_pixel);
        }

        self.pb.finish_with_message("finished rendering image");

        Ok(img_buffer)
    }

    fn get_ray(&self, rng: &mut ThreadRng, i: i64, j: i64) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + Self::pixel_sample_square(self, rng);

        let ray_origin = if self.defocus_angle <= 0. {
            self.lookfrom
        } else {
            Self::defocus_disk_sample(self, rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Point3 {
        let p = Vec3::random_in_unit_disk(rng);
        self.lookfrom + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
