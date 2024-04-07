use tga::tgacolor::Color;
use tga::tgaimage::Image;

use std::mem;
use std::mem::swap;

use crate::color::ColorResolver;
use crate::vertex::Vertex;

pub struct Geometry<'a> {
    pub image: &'a mut Image,
    pub zbuff: Vec<Vec<i32>>,
}

impl<'a> Geometry<'a> {
    pub fn create(image: &'a mut Image) -> Self {
        Self {
            zbuff: vec![vec![i32::MIN; image.width as usize]; image.height as usize],
            image,
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
        if a.y > b.y {
            mem::swap(a, b);
        }
        if a.y > c.y {
            mem::swap(a, c);
        }
        if b.y > c.y {
            mem::swap(b, c);
        }

        self.line(a, b, color);
        self.line(b, c, color);
        self.line(a, c, color);
    }

    pub fn polygon(
        &mut self,
        a: &mut Vertex,
        b: &mut Vertex,
        c: &mut Vertex,
        resolver: &dyn ColorResolver,
        intencity: f32,
    ) {
        if a.y == b.y && a.y == c.y {
            return;
        }

        // Sort triangles by Y
        if a.y > b.y {
            mem::swap(a, b);
        }
        if a.y > c.y {
            mem::swap(a, c);
        }
        if b.y > c.y {
            mem::swap(b, c);
        }

        let total_height = c.y - a.y;
        for i in a.y as isize..=b.y as isize {
            let segment_height = b.y - a.y + 1.0;
            let lslope = (i as f64 - a.y) / total_height;
            let rslope = (i as f64 - a.y) / segment_height;

            let mut x1: Vertex = a.add(&c.sub(&a).mul_by_val(lslope));
            let mut x2: Vertex = a.add(&b.sub(&a).mul_by_val(rslope));

            self.fill_texture(&mut x1, &mut x2, resolver, lslope, rslope, intencity, true);
        }

        for i in b.y as i32..=c.y as i32 {
            let segment_height = c.y - b.y + 1.0;
            let lslope = (i as f64 - a.y) / total_height;
            let rslope = (i as f64 - b.y) / segment_height;

            let mut x1 = a.add(&c.sub(&a).mul_by_val(lslope));
            let mut x2 = b.add(&c.sub(&b).mul_by_val(rslope));

            self.fill_texture(&mut x1, &mut x2, resolver, lslope, rslope, intencity, false);
        }
    }

    fn fill_texture(
        &mut self,
        a: &mut Vertex,
        b: &mut Vertex,
        color_resolver: &dyn ColorResolver,
        lslope: f64,
        rslope: f64,
        intencity: f32,
        up: bool
    ) {
        if a.x > b.x {
            mem::swap(a, b);
        }

        for i in a.x as usize..=b.x as usize {
            let mut phi: f64 = 1.0;

            if b.x != a.x {
                phi = (i as f64 - a.x) / (b.x - a.x);
            }

            let p = a.copy().add(&b.sub(&a).mul_by_val(phi));

            if p.x as usize >= self.image.width || p.y as usize >= self.image.height {
                return;
            }

            let mut color = color_resolver.resolve(lslope, rslope, phi, up);
            color.mul_by_val(intencity);

            if self.zbuff[p.y as usize][p.x as usize] < p.z as i32 {
                self.zbuff[p.y as usize][p.x as usize] = p.z as i32;
                self.image.set_pixel(p.x as isize, p.y as isize, &color);
            }
        }
    }
}
