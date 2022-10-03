use crate::{vec3::Vec3, ray::Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(origin: &Vec3, viewport_height: f64, viewport_width: f64, focal_length: f64) -> Camera {
        Camera { 
            origin: *origin, 
            lower_left_corner: Vec3::new(- viewport_width / 2.0, - viewport_height / 2.0, - focal_length),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin;
        Ray::new(&self.origin, &direction)
    }
}