use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: *origin,
            direction: direction.normal(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::vec3::Vec3;
    #[test]
    fn test_ray_construction() {
        let r = Ray::new(&Vec3::new(3.14, -2.1, 1.4), &Vec3::new(1.0, -2.1, 3.4));
        assert_eq!(r.direction().length(), 1.0)
    }

    #[test]
    fn test_parameter_evaluation() {
        let r = Ray::new(&Vec3::new(3.14, -2.1, 1.4), &Vec3::new(1.0, -2.1, 3.4));

        let p = r.at(0.0);

        assert_eq!(p.x, 3.14);
        assert_eq!(p.y, -2.1);
        assert_eq!(p.z, 1.4);

        let p = r.at(-1.23);

        assert!((p.x - 2.84141).abs() < 0.00001);
        assert!((p.y - -1.47297).abs() < 0.00001);
        assert!((p.z - 0.38481).abs() < 0.00001);
    }
}
