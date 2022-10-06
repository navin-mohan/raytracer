use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    v: Vec3,
    u: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: &Vec3,
        look_at: &Vec3,
        view_up: &Vec3,
        vertical_field_of_view_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = Camera::deg_to_rad(vertical_field_of_view_deg);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).normal();
        let u = (*view_up * w).normal();
        let v = u * w;

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let origin = look_from;

        Camera {
            origin: *origin,
            lower_left_corner: *origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::get_random_point_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let new_origin = self.origin + offset;
        let direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - new_origin;
        Ray::new(&new_origin, &direction)
    }

    fn deg_to_rad(deg: f64) -> f64 {
        deg * std::f64::consts::PI / 180.0
    }
}
