pub const RGB_LEN: usize = 3;

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
}
