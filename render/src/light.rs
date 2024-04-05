use crate::vertex::*;

pub struct Light {}

impl Light {
    pub fn calculate_light(a: &Vertex, b: &Vertex, c: &Vertex) -> Vertex {
        let ac = a.sub(&b);
        let ab = b.sub(&c);
        
        let normal = ac.normal(&ab);
        let v = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
        
        normal.mul_by_val(1.0 / v)
    }
}