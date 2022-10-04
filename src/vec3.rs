use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3::new(
            self.x.sqrt(),
            self.y.sqrt(),
            self.z.sqrt()
        )
    }

    pub fn normal(&self) -> Vec3 {
        Vec3::new(
            self.x,
            self.y,
            self.z
        ) / self.length()
    }

    pub fn get_random_point_on_unit_circle() -> Vec3 {
        Vec3::new(
            fastrand::f64()*2.0 - 1.0,
            fastrand::f64()*2.0 - 1.0,
            fastrand::f64()*2.0 - 1.0
        ).normal()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, v: Self) -> Self {
        Self::new(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z
        )
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, v: f64) -> Self {
        Self::new(
            self.x + v,
            self.y + v,
            self.z + v
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, v: Self) -> Self {
        Self::new(
            self.x - v.x,
            self.y - v.y,
            self.z - v.z
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, v: Self) -> Self {
        Self::new(
            self.y*v.z - self.z*v.y,
            self.x*v.z - self.z*v.x,
            self.x*v.y - self.y*v.x
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, v: f64) -> Self {
        Self::new(
            self.x * v,
            self.y * v,
            self.z * v,
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, v: f64) -> Self::Output {
        Self::new(
            self.x / v,
            self.y / v,
            self.z / v,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_length() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v.length(), 0.0);

        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v.length(), 3.0f64.sqrt());

        let v = Vec3::new(3.14, -9.456, 10.007);
        let delta: f64 = 0.00001;
        let expected_result : f64 = 14.12145;
        assert!((v.length() - expected_result).abs() < delta, "expected {} but got {}, delta={}", expected_result, v.length(), delta);
    }

    #[test]
    fn test_addition() {
        let v1 = Vec3::new(0.0, 3.13, 1.0);
        let v2 = Vec3::new(1.0, 0.0, 2.7123);
        let result = v1 + v2;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 3.13);
        assert_eq!(result.z, 3.7123);
    }

    #[test]
    fn test_subtraction() {
        let v1 = Vec3::new(0.0, 3.13, 1.0);
        let v2 = Vec3::new(1.0, 0.0, 2.7123);
        let result = v1 - v2;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 3.13);
        assert_eq!(result.z, -1.7123);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(0.0, 3.13, 1.0);
        let v2 = Vec3::new(1.0, 0.0, 2.7123);
        let result = v1 * v2;
        assert!(result.x - 8.4895 < 0.0001);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, -3.13);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(0.0, 3.13, 1.0);
        let v2 = Vec3::new(1.0, -1.0, 2.7123);
        let result = v1.dot(&v2);
        assert_eq!(result, -3.13 + 2.7123);
    }

    #[test]
    fn test_normal() {
        let v = Vec3::new(0.0, 3.13, 1.0);
        let result = v.normal();
        assert_eq!(result.x, 0.0);
        assert!((result.y - 0.95256).abs() < 0.00001);
        assert!((result.z - 0.30433).abs() < 0.00001);
    }

    #[test]
    fn test_division() {
        let v = Vec3::new(0.0, 3.13, 1.0);
        let result = v / 2.4;
        assert_eq!(result.x, 0.0);
        assert!((result.y - 1.30416).abs() < 0.00001);
        assert!((result.z - 0.41666).abs() < 0.00001);
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Vec3::new(0.0, 3.13, 1.0);
        let result = v * 2.4;
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 7.512);
        assert_eq!(result.z, 2.4);
    }
}