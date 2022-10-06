use raytracer::{
    camera::Camera,
    hittable:: Hittable,
    vec3::Vec3,
    create_random_scene,
    render
};
use std::{fs::File, io::Write};


fn main() -> std::io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMG_HEIGHT: usize = 400;
    const IMG_WIDTH: usize = (IMG_HEIGHT as f64 * ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: usize = 100;

    const MAX_DEPTH: usize = 50;

    let world: Vec<Box<dyn Hittable>> = create_random_scene();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;

    let camera = Camera::new(
        &look_from,
        &look_at,
        &view_up,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    let image = render(
        IMG_HEIGHT,
        IMG_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        &camera,
        &world,
    );

    let mut f = File::create("test.ppm")?;
    f.write_all(image.to_ppm().as_bytes())?;
    Ok(())
}

