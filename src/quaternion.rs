use vector3::Vector3;

#[derive (Debug, Copy, Clone)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}


impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { w, x, y, z }
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotate_vector(&self, v: &Vector3) -> Vector3 {
        let p = Quaternion::new(0.0, v.x, v.y, v.z);
        let result = *self * p * self.conjugate();
        Vector3::new(result.x, result.y, result.z)
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
        Quaternion { w, x, y, z }
    }
}






