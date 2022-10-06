use crate::{vec3::Vec3, ray::Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3,
    v: Vec3,
    u: Vec3,
    lens_radius: f64,
    focus_dist: f64
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, view_up: &Vec3, vertical_field_of_view_deg: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = Camera::deg_to_rad(vertical_field_of_view_deg);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).normal();
        let u = (*view_up * w).normal();
        let v = u * w;

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let origin = look_from;

        Camera { 
            origin: *look_from, 
            lower_left_corner: *origin - horizontal/2.0 - vertical/2.0 - w*focus_dist,
            horizontal,
            vertical,
            w,
            u,
            v,
            focus_dist,
            lens_radius: aperture/2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::get_random_point_in_unit_disk() * self.lens_radius;
        let offset = u*rd.x + v*rd.y;
        let new_origin = self.origin + offset;
        let direction = self.lower_left_corner + self.horizontal*u + self.vertical*v - new_origin;
        Ray::new(&new_origin, &direction)
    }

    fn deg_to_rad(deg: f64) -> f64 {
        deg  * std::f64::consts::PI / 180.0
    }
}