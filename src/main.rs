mod camera;
mod hittable;
mod image;
mod material;
mod ray;
mod vec3;

use crate::{
    camera::Camera,
    material::Metal,
    material::{Dielectric, Lambertian},
};
use hittable::{sphere::Sphere, HitRecord, Hittable};
use image::{Image, Pixel};
use material::Material;
use ray::Ray;
use std::{fs::File, io::Write, iter::repeat_with, rc::Rc};
use vec3::Vec3;

fn trace_ray(r: &Ray, world: &Vec<Box<dyn Hittable>>, max_depth: usize) -> Pixel {
    if max_depth <= 0 {
        return Pixel::black();
    }

    let mut t_closest_so_far = f64::INFINITY;
    let mut rec: Option<HitRecord> = None;

    for obj in world {
        let result = obj.hit(&r, 0.001, t_closest_so_far);

        if let Some(temp_rec) = result {
            t_closest_so_far = temp_rec.t_value();
            rec = Some(temp_rec);
        }
    }

    if let Some(final_rec) = rec {
        if let Some((attenuation, new_ray)) = final_rec.material().scatter(r, final_rec) {
            let pixel = trace_ray(&new_ray, world, max_depth - 1);

            return Pixel::new(
                (attenuation.x * pixel.r as f64) as u8,
                (attenuation.y * pixel.g as f64) as u8,
                (attenuation.z * pixel.b as f64) as u8,
            );
        }

        return Pixel::black();
    }

    let w = 0.5 * (r.direction().y + 1.0);
    let white: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let blue: Vec3 = Vec3::new(0.5, 0.7, 1.0);
    let color = white * (1.0 - w) + blue * w;
    Pixel::new(
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    )
}

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

fn render(
    image_height: usize,
    image_width: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    camera: &Camera,
    world: &Vec<Box<dyn Hittable>>,
) -> Image {
    let h_range = 0..image_height;
    let w_range = 0..image_width;

    Image::new(
        image_height,
        image_width,
        h_range
            .flat_map(|x| w_range.clone().map(move |y| (image_height - x - 1, y)))
            .map(|(x, y)| {
                repeat_with(|| fastrand::f64())
                    .take(samples_per_pixel)
                    .map(|random_val| {
                        (
                            (y as f64 + random_val) / (image_width as f64 - 1.0),
                            (x as f64 + random_val) / (image_height as f64 - 1.0),
                        )
                    })
                    .map(|(u, v)| camera.get_ray(u, v))
                    .map(|ray| trace_ray(&ray, &world, max_depth))
                    .map(|pixel| Vec3::new(pixel.r as f64, pixel.g as f64, pixel.b as f64))
                    .fold(Vec3::new(0.0, 0.0, 0.0), |acc, v| acc + v)
            }) // anti aliasing
            .map(|pixel| pixel / samples_per_pixel as f64)
            .map(|avg_sample| (avg_sample / 255.0).sqrt() * 256.0)
            .map(|anti_aliased_pixel| Pixel::from_vec3(&anti_aliased_pixel))
            .collect(),
    )
}

fn get_random_material() -> Rc<dyn Material> {
    let materials: Vec<Rc<dyn Material>> = vec![
        Rc::new(Lambertian::new(&Vec3::new(
            fastrand::f64(),
            fastrand::f64(),
            fastrand::f64(),
        ))),
        Rc::new(Metal::new(
            &Vec3::new(
                0.5 + fastrand::f64() / 2.0,
                0.5 + fastrand::f64() / 2.0,
                0.5 + fastrand::f64() / 2.0,
            ),
            fastrand::f64() / 2.0,
        )),
        Rc::new(Dielectric::new(1.5)),
    ];

    materials[fastrand::usize(0..materials.len())].clone()
}

fn create_random_scene() -> Vec<Box<dyn Hittable>> {
    let large_spheres = vec![
        // ground
        Box::new(Sphere::new(
            &Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Rc::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5))),
        )),
        Box::new(Sphere::new(
            &Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Rc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            &Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(&Vec3::new(0.4, 0.2, 0.1))),
        )),
        Box::new(Sphere::new(
            &Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Rc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0)),
        )),
    ];

    let r = -11..11;
    r.clone()
        .flat_map(|a| r.clone().map(move |b| (a, b)))
        .map(|(a, b)| {
            Vec3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            )
        })
        .map(|center| Box::new(Sphere::new(&center, 0.2, get_random_material())))
        .chain(large_spheres.into_iter())
        .collect()
}

impl FromIterator<Box<Sphere>> for Vec<Box<dyn Hittable>> {
    fn from_iter<T: IntoIterator<Item = Box<Sphere>>>(iter: T) -> Self {
        let mut v: Vec<Box<dyn Hittable>> = Vec::new();
        for sphere in iter {
            v.push(sphere);
        }
        v
    }
}
