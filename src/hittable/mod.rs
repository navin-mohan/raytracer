use crate::{ray::Ray, vec3::Vec3};

pub mod sphere;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t_value: f64,
    front_face: bool
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(point: &Vec3, normal: &Vec3, t_value: f64, front_face: bool) -> HitRecord {
        HitRecord { point: *point, normal: normal.normal(), t_value, front_face }
    }

    pub fn t_value(&self) -> f64 {
        self.t_value
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn point(&self) -> &Vec3 {
        &self.point
    }
}