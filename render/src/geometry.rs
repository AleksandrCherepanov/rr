use tga::tgacolor::Color;
use tga::tgaimage::Image;

use std::mem;
use std::mem::swap;

use crate::vertex::Vertex;

pub struct Geometry<'a> {
    pub image: &'a mut Image
}

impl<'a> Geometry<'a> {
    pub fn create(image: &'a mut Image) -> Self {
        Self {
            image
        }
    }

    pub fn line(&mut self, v1: &mut Vertex, v2: &mut Vertex, color: &Color) {
        let mut steep = false;

        let mut x1 = v1.x as isize;
        let mut x2 = v2.x as isize;
        let mut y1 = v1.y as isize;
        let mut y2 = v2.y as isize;

        if x1.abs_diff(x2) < y1.abs_diff(y2) {
           steep = true;
           swap(&mut x1, &mut y1);
           swap(&mut x2, &mut y2);
        }

        if x1 > x2 {
           swap(&mut x1, &mut x2);
           swap(&mut y1, &mut y2);
        }

        let dx = x2 - x1;
        let dy = y2 - y1;
        let derr = dy.abs() * 2;
        let mut err = 0;
        let mut y = y1;
        for x in x1..=x2 { 
            if steep {
                self.image.set_pixel(y, x, color);
            } else {
                self.image.set_pixel(x, y, color);
            }
            err = err + derr;

            if err > dx {
                if y2 > y1 {
                    y = y + 1;
                } else {
                    y = y - 1;
                }
                err = err - dx * 2;
            }
        }
    }

    pub fn triangle(&mut self, a: &mut Vertex, b: &mut Vertex, c: &mut Vertex, color: &Color) {
        if a.y == b.y && a.y == c.y {
            return;
        }

        // Sort triangles by Y
        if a.y > b.y { mem::swap(a, b); }
        if a.y > c.y { mem::swap(a, c); }
        if b.y > c.y { mem::swap(b, c); }

        self.line(a, b, color); 
        self.line(b, c, color); 
        self.line(a, c, color); 
    }
    
    pub fn polygon(&mut self, a: &mut Vertex, b: &mut Vertex, c: &mut Vertex, zbuff: &mut Vec<Vec<i32>>, texture: &Vec<Vertex>, map: &Image, intencity: f32) {
        if a.y == b.y && a.y == c.y {
            return;
        }

        let mut tx1 = &texture[0];
        let mut tx2 = &texture[1];
        let mut tx3 = &texture[2];

        // Sort triangles by Y
        if a.y > b.y { mem::swap(a, b); }
        if a.y > c.y { mem::swap(a, c); }
        if b.y > c.y { mem::swap(b, c); }
        
        if tx1.y > tx2.y { mem::swap(&mut tx1, &mut tx2); }
        if tx1.y > tx3.y { mem::swap(&mut tx1, &mut tx3); }
        if tx2.y > tx3.y { mem::swap(&mut tx2, &mut tx3); }

        let total_height = c.y - a.y;
        for i in a.y as isize..=b.y as isize {
            let segment_height = b.y - a.y + 1.0;
            let slope_left = (i as f64 - a.y) / total_height; 
            let slope_right = (i as f64 - a.y) / segment_height;

            let mut x1: Vertex = a.add(&c.sub(&a).mul_by_val(slope_left));
            let mut x2: Vertex = a.add(&b.sub(&a).mul_by_val(slope_right));
            
            let mut txx1: Vertex = tx1.add(&tx3.sub(&tx1).mul_by_val(slope_left));
            let mut txx2: Vertex = tx1.add(&tx2.sub(&tx1).mul_by_val(slope_right));

            self.fill_between(&mut x1, &mut x2, zbuff, &mut txx1, &mut txx2, map, intencity);
        }
        
        for i in b.y as i32..=c.y as i32 {
            let segment_height = c.y - b.y + 1.0;
            let slope_left = (i as f64 - a.y) / total_height; 
            let slope_right = (i as f64 - b.y) / segment_height;

            let mut x1 = a.add(&c.sub(&a).mul_by_val(slope_left));
            let mut x2 = b.add(&c.sub(&b).mul_by_val(slope_right));

            let mut txx1: Vertex = tx1.add(&tx3.sub(&tx1).mul_by_val(slope_left));
            let mut txx2: Vertex = tx2.add(&tx3.sub(&tx2).mul_by_val(slope_right));

            self.fill_between(&mut x1, &mut x2, zbuff, &mut txx1, &mut txx2, map, intencity);
        }
    }

    fn fill_between(&mut self, a: &mut Vertex, b: &mut Vertex, zbuff: &mut Vec<Vec<i32>>, tx1: &mut Vertex, tx2: &mut Vertex, map: &Image, intencity: f32) {
        if a.x > b.x {
            mem::swap(a, b);
        }
        
        for i in a.x as usize..=b.x as usize {
            let mut phi: f64 = 1.0; 
            
            if b.x != a.x {
                phi = (i as f64 - a.x) / (b.x - a.x);
            }
            
            let p = a.copy().add(&b.sub(&a).mul_by_val(phi));
            let t = tx1.copy().add(&tx2.sub(&tx1).mul_by_val(phi));
            if p.x as usize >= self.image.width || p.y as usize >= self.image.height {
                return;
            }
            if zbuff[p.y as usize][p.x as usize] < p.z as i32 {
                zbuff[p.y as usize][p.x as usize] = p.z as i32;
                let color = map.get_pixel(t.x as isize, t.y as isize);
                let color = Color::create(
                    (color.bytes[2] as f32 * intencity) as u8, 
                    (color.bytes[1] as f32 * intencity) as u8, 
                    (color.bytes[0] as f32 * intencity) as u8,
                );
                self.image.set_pixel(p.x as isize, p.y as isize, &color);
            }
        }
    }
}
