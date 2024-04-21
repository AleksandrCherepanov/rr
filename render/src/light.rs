use crate::vertex::*;

pub struct Light {}

impl Light {
    pub fn calculate_light(a: &Vertex, b: &Vertex, c: &Vertex) -> Vertex {
        let ac = a - b;
        let bc = b - c;

        let normal = ac.normal(&bc);
        let v = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();

        normal * (1.0 / v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_light() {
        let v1 = Vertex::create(5.0, 25.0, 10.0);
        let v2 = Vertex::create(15.0, 5.0, 20.0);
        let v3 = Vertex::create(10.0, 30.0, 5.0);

        let l = Light::calculate_light(&v1, &v2, &v3);
        assert_eq!(0.2672612419124244, l.x);
        assert_eq!(0.5345224838248488, l.y);
        assert_eq!(0.8017837257372732, l.z);
    }
}
