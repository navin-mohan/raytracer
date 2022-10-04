use crate::{vec3::Vec3, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r: &Ray, hit_record: HitRecord) -> Option<(Vec3,Ray)>;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hit_record: HitRecord) -> Option<(Vec3,Ray)> {
        let scatter_direction = *hit_record.normal() + Vec3::get_random_point_on_unit_circle();
        if scatter_direction.is_near_zero() {
            Some((
                self.albedo,
                Ray::new(hit_record.point(), hit_record.normal())
            ))
        } else {
            Some((
                self.albedo,
                Ray::new(hit_record.point(), &scatter_direction)
            ))
        }
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: &Vec3) -> Metal {
        Metal { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: HitRecord) -> Option<(Vec3,Ray)> {
        let reflected_direction = reflect(r.direction(), hit_record.normal());
        if reflected_direction.dot(hit_record.normal()) > 0.0 {
            let reflected_ray = Ray::new(
                hit_record.point(),
                &reflected_direction
            );
            Some((
                self.albedo,
                reflected_ray
            ))
        } else {
            None
        }
    }
}

fn reflect(v_in: &Vec3, normal: &Vec3) -> Vec3 {
    *v_in - *normal*2.0*v_in.dot(normal)
}