use crate::{material::Material, ray::Ray, vec3::Vec3};
use std::rc::Rc;

pub mod sphere;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t_value: f64,
    front_face: bool,
    material: Rc<dyn Material>,
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(
        point: &Vec3,
        normal: &Vec3,
        t_value: f64,
        front_face: bool,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            point: *point,
            normal: normal.normal(),
            t_value,
            front_face,
            material,
        }
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

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}
