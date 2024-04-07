use phf::phf_map;

pub const RGB_LEN: usize = 3;

pub const RED: [u8; 3] = [255, 0, 0];
pub const ORANGE: [u8; 3] = [255, 165, 0];
pub const YELLOW: [u8; 3] = [255, 255, 0];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const BLUE: [u8; 3] = [173, 216, 230];
pub const DARK_BLUE: [u8; 3] = [0, 0, 255];
pub const PURPLE: [u8; 3] = [160, 32, 240];
pub const WHITE: [u8; 3] = [255, 255, 255];

pub const DEFAULT_COLOR: [u8; 3] = WHITE;

pub static AVAILABLE_COLORS: phf::Map<&'static str, [u8; 3]> = phf_map! {
	"red" => RED,
	"orange" => ORANGE,
	"yellow" => YELLOW,
	"green" => GREEN,
	"blue" => BLUE,
	"dark_blue" => DARK_BLUE,
	"purple" => PURPLE,
	"white" => WHITE
};

#[derive(Copy, Clone)]
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

	pub fn get_bytes(&self) -> &[u8] {
		&self.bytes[..]
	}

	pub fn mul_by_val(&mut self, val: f32) {
		self.bytes[0] = (self.bytes[0] as f32 * val) as u8;
		self.bytes[1] = (self.bytes[1] as f32 * val) as u8;
		self.bytes[2] = (self.bytes[2] as f32 * val) as u8;
	}
}