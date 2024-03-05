use simple_ray_tracer::{Camera, Image, Point3, Vec3, World};

const OUT_FILE_NAME: &str = "output.png";

fn main() -> simple_ray_tracer::Result<()> {
    let world = World::scene();

    let img = Image::new(1200, 16. / 9.);

    let cam = Camera::new(
        img,
        10,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::ZERO,
        Vec3::Y,
        0.6,
        10.,
    );

    let image = cam.render(world)?;

    image.save(OUT_FILE_NAME)?;

    Ok(())
}
