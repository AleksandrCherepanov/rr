use tga::{tgacolor::Color as TgaColor, tgaimage::Image};
use std::mem;

use crate::vertex::Vertex;

pub trait ColorResolver {
    fn resolve(&self, lslope: f64, rslope: f64, phi: f64, up: bool) -> TgaColor;
}

pub struct Texture<'a> {
    pub color_pos: &'a Vec<Vertex>, 
    pub texture: &'a Image,
}

impl<'a> ColorResolver for Texture<'a> {
    fn resolve(&self, lslope: f64, rslope: f64, phi: f64, up: bool) -> TgaColor {
        let mut tx1 = &self.color_pos[0];
        let mut tx2 = &self.color_pos[1];
        let mut tx3 = &self.color_pos[2];

        if tx1.y > tx2.y { mem::swap(&mut tx1, &mut tx2); }
        if tx1.y > tx3.y { mem::swap(&mut tx1, &mut tx3); }
        if tx2.y > tx3.y { mem::swap(&mut tx2, &mut tx3); }

        let txx1: Vertex;
        let txx2: Vertex;

        if up {
            txx1 = tx1.add(&tx3.sub(&tx1).mul_by_val(lslope));
            txx2 = tx1.add(&tx2.sub(&tx1).mul_by_val(rslope));
        } else {
            txx1 = tx1.add(&tx3.sub(&tx1).mul_by_val(lslope));
            txx2 = tx2.add(&tx3.sub(&tx2).mul_by_val(rslope));
        }

        let t = txx1.copy().add(&txx2.sub(&txx1).mul_by_val(phi));

        let color = self.texture.get_pixel(t.x as isize, t.y as isize);
        
        TgaColor::create(
            (color.bytes[2]) as u8, 
            (color.bytes[1]) as u8, 
            (color.bytes[0]) as u8,
        )
    }
}

pub struct Color {
    pub color: TgaColor
}

impl ColorResolver for Color {
    fn resolve(&self, _lslope: f64, _rslope: f64, _phi: f64, _up: bool) -> TgaColor {
        self.color
    }     
}
