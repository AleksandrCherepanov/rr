use tga::{tgacolor::Color as TgaColor, tgaimage::Image};
use std::mem;

use crate::vertex::Vertex;

pub trait ColorResolver {
    fn resolve(&mut self, phi: f64) -> TgaColor;
    fn calculate_texture_pos(&mut self, lslope: f64, rslope: f64, up: bool);
    fn swap(&mut self);
    fn adjust_ab(&mut self);
    fn adjust_ac(&mut self);
    fn adjust_bc(&mut self);
}

pub struct Texture<'a> {
    pub v1: &'a Vertex,
    pub v2: &'a Vertex,
    pub v3: &'a Vertex,
    pub texture: &'a Image,
    pub texture_a: Vertex,
    pub texture_b: Vertex,
}

impl<'a> ColorResolver for Texture<'a> {
    fn adjust_ab(&mut self) {
        mem::swap(&mut self.v1, &mut self.v2);
    }
    fn adjust_ac(&mut self) {
        mem::swap(&mut self.v1, &mut self.v3);
    }
    fn adjust_bc(&mut self) {
        mem::swap(&mut self.v2, &mut self.v3);
    }

    fn calculate_texture_pos(&mut self, lslope: f64, rslope: f64, up: bool) {
        let tx1 = self.v1;
        let tx2 = &self.v2;
        let tx3 = &self.v3;

        let txx1: Vertex;
        let txx2: Vertex;

        if up {
            txx1 = tx1.add(&tx3.sub(&tx1).mul_by_val(lslope));
            txx2 = tx1.add(&tx2.sub(&tx1).mul_by_val(rslope));
        } else {
            txx1 = tx1.add(&tx3.sub(&tx1).mul_by_val(lslope));
            txx2 = tx2.add(&tx3.sub(&tx2).mul_by_val(rslope));
        }

        self.texture_a = txx1;
        self.texture_b = txx2;
    }

    fn resolve(&mut self, phi: f64) -> TgaColor {
        let t = self.texture_a.copy().add(&self.texture_b.sub(&self.texture_a).mul_by_val(phi));
        let color = self.texture.get_pixel(t.x as isize, t.y as isize);
        TgaColor::create(
            (color.bytes[2]) as u8,
            (color.bytes[1]) as u8,
            (color.bytes[0]) as u8,
        )
    }

    fn swap(&mut self) {
        mem::swap(&mut self.texture_a, &mut self.texture_b);
    }
}

pub struct Color {
    pub color: TgaColor,
}

impl ColorResolver for Color {
    fn resolve(&mut self, _phi: f64) -> TgaColor {
        self.color
    }

    fn calculate_texture_pos(&mut self, _lslope: f64, _rslope: f64, _up: bool) {}

    fn swap(&mut self) {}

    fn adjust_ab(&mut self) {}

    fn adjust_ac(&mut self) {}

    fn adjust_bc(&mut self) {}
}
