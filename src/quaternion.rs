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

    pub fn from_axis_angle(axis: &Vector3, angle: f64) -> Quaternion {
        let half_angle = angle / 2.0;
        let sin_half_angle = half_angle.sin();
        let cos_half_angle = half_angle.cos();
        let x = axis.x * sin_half_angle;
        let y = axis.y * sin_half_angle;
        let z = axis.z * sin_half_angle;
        let w = cos_half_angle;
        Quaternion { w, x, y, z }
    }

    pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
        let half_roll = roll / 2.0;
        let sin_half_roll = half_roll.sin();
        let cos_half_roll = half_roll.cos();
        let half_pitch = pitch / 2.0;
        let sin_half_pitch = half_pitch.sin();
        let cos_half_pitch = half_pitch.cos();
        let half_yaw = yaw / 2.0;
        let sin_half_yaw = half_yaw.sin();
        let cos_half_yaw = half_yaw.cos();
        let w = cos_half_roll * cos_half_pitch * cos_half_yaw + sin_half_roll * sin_half_pitch * sin_half_yaw;
        let x = sin_half_roll * cos_half_pitch * cos_half_yaw - cos_half_roll * sin_half_pitch * sin_half_yaw;
        let y = cos_half_roll * sin_half_pitch * cos_half_yaw + sin_half_roll * cos_half_pitch * sin_half_yaw;
        let z = cos_half_roll * cos_half_pitch * sin_half_yaw - sin_half_roll * sin_half_pitch * cos_half_yaw;
        Quaternion { w, x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.w /= len;
        self.x /= len;
        self.y /= len;
        self.z /= len;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 2.0);
        assert_eq!(q.y, 3.0);
        assert_eq!(q.z, 4.0);
    }

}






