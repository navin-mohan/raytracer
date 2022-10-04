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

        let cos_theta = (*r.direction()*-1.0).dot(&hit_record.normal());
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let refraction_ratio = if hit_record.front_face() { 1.0 / self.refractive_index } else { self.refractive_index };

        let new_direction = if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > fastrand::f64() {
            // no solution for snell's law. Hence, no refraction
            reflect(r.direction(), hit_record.normal())
        } else {
            refract(&r.direction().normal(), hit_record.normal(), refraction_ratio)
        };
        Some((
            Vec3::new(1.0, 1.0, 1.0),
            Ray::new(hit_record.point(), &new_direction)
        ))
    }
}

fn refract(v_in: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (v_in.normal()*-1.0).dot(&normal.normal());
    let r_out_perpendicular = (*v_in + *normal*cos_theta)*etai_over_etat;
    let r_out_parallel = *normal * -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt());
    r_out_parallel + r_out_perpendicular
}

fn reflectance(cosine: f64, ref_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r0 = r0*r0;
    r0 + (1.0 - r0)*((1.0 - cosine).powi(5))
}