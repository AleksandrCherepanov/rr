use tga::{tgacolor::Color as TgaColor, tgaimage::Image};
use std::mem;

use crate::vertex::Vertex;

pub trait ColorResolver {
    fn resolve(&mut self, phi: f64) -> TgaColor;
    fn debug(&self);
    fn calculate_texture_pos(&mut self, lslope: f64, rslope: f64, up: bool);

    fn swap(&mut self);
}

pub struct Texture<'a> {
    pub color_pos: &'a Vec<Vertex>,
    pub texture: &'a Image,
    pub texture_a: Vertex,
    pub texture_b: Vertex,
}

impl<'a> ColorResolver for Texture<'a> {
    fn calculate_texture_pos(&mut self, lslope: f64, rslope: f64, up: bool) {
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

    fn debug(&self) {
        println!("{:?}", self.color_pos)
    }
}

pub struct Color {
    pub color: TgaColor,
}

impl ColorResolver for Color {
    fn resolve(&mut self, _phi: f64) -> TgaColor {
        self.color
    }

    fn debug(&self) {
        todo!()
    }

    fn calculate_texture_pos(&mut self, _lslope: f64, _rslope: f64, _up: bool) {}

    fn swap(&mut self) {}
}
