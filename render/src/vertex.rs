use std::ops::{Mul, Sub, Add};

#[derive(Debug, PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Mul<&Vertex> for &Vertex {
    type Output = Vertex;

    fn mul(self, rhs: &Vertex) -> Vertex {
        Vertex {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f64) -> Vertex {
        Vertex {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub<&Vertex> for &Vertex {
    type Output = Vertex;

    fn sub(self, rhs: &Vertex) -> Vertex {
        Vertex {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<&Vertex> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: &Vertex) -> Vertex {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Vertex {
    pub fn create(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn create_zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn create_from_string(data: &str) -> Self {
        let mut coords = data.split_whitespace()
            .map(|el| el.parse::<f64>().expect("can't parse line to vertex"));

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap_or_default(),
        }
    }

    //TODO maybe I should use derive Clone instead of this method
    pub fn copy(&self) -> Self {
        Self { x: self.x, y: self.y, z: self.z }
    }

    //TODO replace all simple operation by traits implementation
    pub fn sub(&self, p: &Vertex) -> Vertex {
        let x = self.x - p.x;
        let y = self.y - p.y;
        let z = self.z - p.z;

        Vertex::create(x, y, z)
    }

    pub fn add(&self, p: &Vertex) -> Vertex {
        let x = self.x + p.x;
        let y = self.y + p.y;
        let z = self.z + p.z;

        Vertex::create(x, y, z)
    }

    pub fn mul_by_val(&self, val: f64) -> Vertex {
        let dx: f64 = self.x * val;
        let dy: f64 = self.y * val;
        let dz: f64 = self.z * val;

        Vertex::create(dx, dy, dz)
    }

    pub fn normal(&self, p: &Vertex) -> Vertex {
        Vertex::create(self.y * p.z - self.z * p.y, self.z * p.x - self.x * p.z, self.x * p.y - self.y * p.x)
    }

    //TODO rename to dot product, change type to f64, better to convert in place of usage
    pub fn mul_scalar(&self, p: &Vertex) -> f32 {
        (self.x * p.x + self.y * p.y + self.z * p.z) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let v = Vertex::create(1.0, 2.0, 3.0);
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    fn create_zero() {
        let v = Vertex::create_zero();
        assert_eq!(0.0, v.x);
        assert_eq!(0.0, v.y);
        assert_eq!(0.0, v.z);
    }

    #[test]
    fn create_from_string() {
        let v = Vertex::create_from_string("1.0 2.0 3.0");
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    #[should_panic]
    fn create_from_string_fail() {
        let v = Vertex::create_from_string("1.02.03.0");
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    fn add() {
        let v1 = Vertex::create(1.0, 2.0, 3.0);
        let v2 = Vertex::create(7.0, 12.0, 30.0);

        let v3 = v1 + &v2;
        assert_eq!(8.0, v3.x);
        assert_eq!(14.0, v3.y);
        assert_eq!(33.0, v3.z);
    }

    #[test]
    fn sub() {
        let v1 = Vertex::create(1.0, 2.0, 3.0);
        let v2 = Vertex::create(7.0, 12.0, 30.0);

        let v3 = &v2 - &v1;
        assert_eq!(6.0, v3.x);
        assert_eq!(10.0, v3.y);
        assert_eq!(27.0, v3.z);
    }

    #[test]
    fn mul() {
        let v1 = Vertex::create(1.0, 2.0, 3.0);
        let v2 = Vertex::create(7.0, 12.0, 30.0);

        let v3 = &v1 * &v2;
        assert_eq!(7.0, v3.x);
        assert_eq!(24.0, v3.y);
        assert_eq!(90.0, v3.z);

        let v1 = Vertex::create(1.0, 2.0, 3.0);
        let v3 = v1 * 3.0;
        assert_eq!(3.0, v3.x);
        assert_eq!(6.0, v3.y);
        assert_eq!(9.0, v3.z);
    }

    #[test]
    fn mul_scalar() {
        let v1 = Vertex::create(1.0, 2.0, 3.0);
        let v2 = Vertex::create(7.0, 12.0, 30.0);

        let r = v1.mul_scalar(&v2);
        assert_eq!(121.0, r);
    }

    #[test]
    fn normal() {
        let v1 = Vertex::create(1.0, 2.0, 3.0);
        let v2 = Vertex::create(7.0, 12.0, 30.0);

        let n = v1.normal(&v2);
        assert_eq!(24.0, n.x);
        assert_eq!(-9.0, n.y);
        assert_eq!(-2.0, n.z);
    }
}
