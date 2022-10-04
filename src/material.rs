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
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: f64) -> Metal {
        Metal { albedo: *albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: HitRecord) -> Option<(Vec3,Ray)> {
        let reflected_direction = reflect(r.direction(), hit_record.normal());
        if reflected_direction.dot(hit_record.normal()) > 0.0 {
            let reflected_ray = Ray::new(
                hit_record.point(),
                &(reflected_direction + Vec3::get_random_point_on_unit_circle()*self.fuzz)
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

pub struct Dielectric {
    refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit_record: HitRecord) -> Option<(Vec3,Ray)> {
        let etai_over_etat = if hit_record.front_face() { 1.0 / self.refractive_index } else { self.refractive_index };
        let refracted_direction = refract(&r.direction().normal(), hit_record.normal(), etai_over_etat);
        Some((
            Vec3::new(1.0, 1.0, 1.0),
            Ray::new(hit_record.point(), &refracted_direction)
        ))
    }
}

fn refract(v_in: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (v_in.normal()*-1.0).dot(&normal.normal());
    let r_out_perpendicular = (*v_in + *normal*cos_theta)*etai_over_etat;
    let r_out_parallel = *normal * -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt());
    r_out_parallel + r_out_perpendicular
}