
use crate::{vec3::Vec3, ray::Ray};
use super::{Hittable, HitRecord};
pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Sphere {
        Sphere { center: *center, radius }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        (*point - *self.center()).normal()
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = *r.origin() - *self.center();
        let b_half = origin_to_center.dot(r.direction()); 
        let a = r.direction().length_squared();
        let c = origin_to_center.length_squared() - self.radius*self.radius;
        let determinant = b_half*b_half - a*c;
        if determinant >= 0.0 {
            let determinant_sqrt = determinant.sqrt();
            let mut t = (-b_half - determinant_sqrt) / a;

            if t < t_min || t > t_max {
                t = (-b_half + determinant_sqrt) / a;
            }
            
            if t >= t_min && t <= t_max {
                let point = r.at(t);
                let outward_normal = self.get_normal(&point);
                let front_face = r.direction().dot(&outward_normal) < 0.0;
                let normal = if front_face { outward_normal } else { outward_normal*-1.0 };
                return Some(HitRecord::new(&point, &normal, t, front_face))
            }
        }

        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::{ray::Ray, vec3::Vec3, hittable::Hittable};

    #[test]
    fn test_hit() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let t_min: f64 = 0.0;
        let t_max: f64 = f64::INFINITY;

        let center = Vec3::new(0.0, 0.0, -5.0);
        let radius: f64 = 2.0;
        let sphere = Sphere::new(&center, radius);

        let direction = Vec3::new(0.0, radius + 1.0, -5.0);
        let ray_miss = Ray::new(&origin, &direction);

        assert!(sphere.hit(&ray_miss, t_min, t_max).is_none());

        let direction = Vec3::new(0.0, radius - 1.0, -5.0) - origin;
        let ray_hit = Ray::new(&origin, &direction);
        assert!(sphere.hit(&ray_hit, t_min, t_max).is_some());
    }
}