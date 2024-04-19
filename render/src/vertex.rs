#[derive(Debug)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn create_from_string(data: &str) -> Self {
        let mut coords = data.split_whitespace()
            .map(|el| el.parse::<f64>().expect("can't parese line to vertex"));

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap_or_default(),
        }
    }

    pub fn create(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn create_zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn copy(&self) -> Self {
        Self { x: self.x, y: self.y, z: self.z }
    }

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

    pub fn normal(&self, p: &Vertex) -> Vertex {
        Vertex::create(self.y * p.z - self.z * p.y, self.z * p.x - self.x * p.z, self.x * p.y - self.y * p.x)
    }

    pub fn mul_by_val(&self, val: f64) -> Vertex {
        let dx: f64 = self.x * val;
        let dy: f64 = self.y * val;
        let dz: f64 = self.z * val;

        Vertex::create(dx, dy, dz)
    }

    pub fn mul_scalar(&self, p: &Vertex) -> f32 {
        (self.x * p.x + self.y * p.y + self.z * p.z) as f32
    }
}