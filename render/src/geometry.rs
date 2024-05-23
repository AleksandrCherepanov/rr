use tga::tgacolor::Color;
use tga::tgaimage::Image;

use std::mem;

use crate::color::ColorResolver;
use crate::vertex::Vertex;
use crate::math;

pub struct Geometry<'a> {
    pub image: &'a mut Image,
    pub zbuff: Vec<Vec<i32>>,
}

impl<'a> Geometry<'a> {
    pub fn create(image: &'a mut Image) -> Self {
        Self {
            zbuff: vec![vec![i32::MIN; image.width]; image.height],
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
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
        }

        if x1 > x2 {
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
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
        resolver: &mut dyn ColorResolver,
        intencity: f32,
    ) {
        if a.y == b.y && a.y == c.y {
            return;
        }

        // Sort triangles by Y
        if a.y > b.y {
            mem::swap(a, b);
            resolver.adjust_ab();
        }
        if a.y > c.y {
            mem::swap(a, c);
            resolver.adjust_ac();
        }
        if b.y > c.y {
            mem::swap(b, c);
            resolver.adjust_bc();
        }

        let total_height = c.y - a.y;
        for i in a.y as isize..b.y as isize {
            let segment_height = b.y - a.y + 1.0;
            let lslope = math::slope(i as f64, a.y, total_height);
            let rslope = math::slope(i as f64, a.y, segment_height);

            let mut x1: Vertex = a.add(&c.sub(&a).mul_by_val(lslope));
            let mut x2: Vertex = a.add(&b.sub(&a).mul_by_val(rslope));

            resolver.calculate_texture_pos(lslope, rslope, true);
            self.fill_texture(&mut x1, &mut x2, resolver, intencity);
        }

        for i in b.y as i32..c.y as i32 {
            let segment_height = c.y - b.y + 1.0;
            let lslope = math::slope(i as f64, a.y, total_height);
            let rslope = math::slope(i as f64, b.y, segment_height);

            let mut x1 = a.add(&c.sub(&a).mul_by_val(lslope));
            let mut x2 = b.add(&c.sub(&b).mul_by_val(rslope));

            resolver.calculate_texture_pos(lslope, rslope, false);
            self.fill_texture(&mut x1, &mut x2, resolver, intencity);
        }
    }

    fn fill_texture(
        &mut self,
        a: &mut Vertex,
        b: &mut Vertex,
        color_resolver: &mut dyn ColorResolver,
        intencity: f32,
    ) {
        if a.x > b.x {
            mem::swap(a, b);
            color_resolver.swap();
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

            let mut color = color_resolver.resolve(phi);
            color.mul(intencity);

            if self.zbuff[a.y as usize][p.x as usize] < p.z as i32 {
                self.zbuff[a.y as usize][p.x as usize] = p.z as i32;
                self.image.set_pixel(p.x as isize, a.y as isize, &color);
            }
        }
    }
}
