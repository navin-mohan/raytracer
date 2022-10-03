mod vec3;
mod image;
mod ray;

use image::{Image, Pixel};
use vec3::Vec3;
use ray::Ray;


fn trace_ray(r: &Ray, origin: &Vec3) -> Pixel {
    let c = Circle::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let points = c.find_intersection_points(r);
    if !points.is_empty() {
        let mut closest_point: Vec3 = points[0];
        let mut dist_closest_point = (points[0] - *origin).length_squared();
        for point in points {
            let distance_of_point = (point - *origin).length_squared();
            if dist_closest_point > distance_of_point {
                closest_point = point;
                dist_closest_point = distance_of_point;
            }
        }

        let sphere_normal = ((closest_point - *c.center()).normal() + 1.0) * 0.5;

        return Pixel::new(
            (sphere_normal.x * 255.0) as u8,
            (sphere_normal.y * 255.0) as u8,
            (sphere_normal.z * 255.0) as u8
        );
    }

    // let c = Circle::new(Vec3::new(1.0, 0.0, -1.0), 0.2);
    // let points = c.find_intersection_points(r);
    // if !points.is_empty() {
    //     return Pixel::new(0, 255, 0);

    // }

    // let c = Circle::new(Vec3::new(-1.0, 0.0, -1.0), 0.2);
    // let points = c.find_intersection_points(r);
    // if !points.is_empty() {
    //     return Pixel::new(0, 0, 255);

    // }

    let w = 0.5*(r.direction().y + 1.0);
    let white: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let blue: Vec3 = Vec3::new(0.5, 0.7, 1.0);
    let color = white*(1.0 - w) + blue*w;
    Pixel::new(
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8
    )
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;

    const VP_HEIGHT: f64 = 2.0;
    const VP_WIDTH: f64 = VP_HEIGHT * ASPECT_RATIO;

    const IMG_HEIGHT: usize = 400;
    const IMG_WIDTH: usize = (IMG_HEIGHT as f64 * ASPECT_RATIO) as usize;

    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(- VP_WIDTH / 2.0, - VP_HEIGHT / 2.0, - FOCAL_LENGTH);
    let horizontal = Vec3::new(VP_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VP_HEIGHT, 0.0);

    let mut image = Image::new(IMG_HEIGHT, IMG_WIDTH);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let u: f64 = x as f64 / (image.width() as f64 - 1.0);
            let v: f64 = y as f64 / (image.height() as f64 - 1.0);
            let direction = lower_left_corner + horizontal*u + vertical*v - origin;
            let ray = Ray::new(origin, &direction);
            let pixel = trace_ray(&ray, &origin);
            image.insert_pixel_at(x, image.height() - y - 1, &pixel);
        }
    }

    print!("{}", image.to_ppm())
}

struct Circle {
    center: Vec3,
    radius: f64
}

impl Circle {
    pub fn new(center: Vec3, radius: f64) -> Circle {
        Circle { center, radius }
    }

    pub fn find_intersection_points(&self, r: &Ray) -> Vec<Vec3> {
        let origin_to_center = self.center - *r.origin();
        let b = 2.0 * origin_to_center.dot(r.direction()); 
        let a = r.direction().length_squared();
        let c = origin_to_center.length_squared() - self.radius*self.radius;
        let determinant = b*b - 4.0*a*c;

        let mut v = Vec::new();

        if determinant == 0.0 {
            let t = -b / (2.0*a);
            v.push(r.at(t));
        } else if determinant > 0.0 {
            let t1 = (-b + determinant) / (2.0*a);
            let t2 = (-b - determinant) / (2.0*a);
            v.push(r.at(t1));
            v.push(r.at(t2));
        }
        v
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }
}