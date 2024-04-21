use phf::phf_map;
//TODO rewrite this module using traits
//Color -> RGB, GBR
//Add traits to enums and try to remove AVAILABLE_COLORS
//Try to use enums from CLI
//Rename files without tga prefix as it already defined in the folder
//Should always use RGB order and only writer should decide how to keep it in file
//I think color and image files should be in render, in tga we should keep only header and reader/writer
//All color is GBR now (needs to be refactored)
// pub const RED: [u8; RGB_LEN] = [255, 0, 0];
pub const RED: [u8; RGB_LEN] = [0, 0, 255];
pub const ORANGE: [u8; RGB_LEN] = [255, 165, 0];
// pub const YELLOW: [u8; RGB_LEN] = [255, 255, 0];
pub const YELLOW: [u8; RGB_LEN] = [0, 255, 255];
pub const GREEN: [u8; RGB_LEN] = [0, 255, 0];
pub const BLUE: [u8; RGB_LEN] = [173, 216, 230];
pub const DARK_BLUE: [u8; RGB_LEN] = [0, 0, 255];
pub const PURPLE: [u8; RGB_LEN] = [160, 32, 240];
pub const WHITE: [u8; RGB_LEN] = [255, 255, 255];
pub const DEFAULT_COLOR: [u8; RGB_LEN] = WHITE;
pub const RGB_LEN: usize = 3;

pub static AVAILABLE_COLORS: phf::Map<&'static str, [u8; RGB_LEN]> = phf_map! {
	"red" => RED,
	"orange" => ORANGE,
	"yellow" => YELLOW,
	"green" => GREEN,
	"blue" => BLUE,
	"dark_blue" => DARK_BLUE,
	"purple" => PURPLE,
	"white" => WHITE
};

//TODO remove pub and derive
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub bytes: [u8; RGB_LEN],
}

impl Color {
    pub fn create(r: u8, g: u8, b: u8) -> Self {
        let mut color = Self { bytes: [0; RGB_LEN] };
        color.bytes[0] = b;
        color.bytes[1] = g;
        color.bytes[2] = r;

        color
    }

    pub fn create_from_bytes(bytes: [u8; RGB_LEN]) -> Self {
        Self {
            bytes
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn mul(&mut self, val: f32) {
        self.bytes[0] = (self.bytes[0] as f32 * val) as u8;
        self.bytes[1] = (self.bytes[1] as f32 * val) as u8;
        self.bytes[2] = (self.bytes[2] as f32 * val) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_create() {
        let c = Color::create(10, 20, 30);
        assert_eq!([30, 20, 10], c.to_bytes(), "should set bytes in reverse order");
    }

    #[test]
    fn color_create_from_bytes() {
        let c = Color::create_from_bytes([10, 20, 30]);
        assert_eq!([10, 20, 30], c.to_bytes(), "should set bytes as is");
    }

    #[test]
    fn color_mul_by_val() {
        let mut c1 = Color::create(10, 20, 30);
        c1.mul(1.5);

        let mut c2 = Color::create_from_bytes([10, 20, 30]);
        c2.mul(1.5);

        assert_eq!([45, 30, 15], c1.to_bytes(), "should multiply all bytes not changing order");
        assert_eq!([15, 30, 45], c2.to_bytes(), "should multiply all bytes not changing order");
    }
}