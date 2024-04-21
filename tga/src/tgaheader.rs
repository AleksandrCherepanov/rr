//const ID_LENGTH: usize = 0;
//const COLOR_MAP_TYPE: usize = 1;
const DATA_TYPE_CODE: usize = 2;
//const COLOR_MAP_ORIGIN_LO: usize = 3;
//const COLOR_MAP_ORIGIN_HI: usize = 4;
//const COLOR_MAP_LENGTH_LO: usize = 5;
//const COLOR_MAP_LENGTH_HI: usize = 6;
//const COLOR_MAP_DEPTH: usize = 7;
//const X_ORIGIN_LO: usize = 8;
//const X_ORIGIN_HI: usize = 9;
//const Y_ORIGIN_LO: usize = 10;
//const Y_ORIGIN_HI: usize = 11;
const WIDTH_LO: usize = 12;
const WIDTH_HI: usize = 13;
const HEIGHT_LO: usize = 14;
const HEIGHT_HI: usize = 15;
const BITS_PER_PIXEL: usize = 16;
const IMAGE_DESCRIPTOR: usize = 17;
const UNCOMPRESSED_RGB: u8 = 2;
const TOP_TO_BOTTOM_ORDER: u8 = 32;

pub const HEADER_SIZE: usize = 18;

pub struct Header {
    bytes: [u8; 18],
}

impl Header {
    pub fn create(
        width: u16,
        height: u16,
    ) -> Self {
        let mut header = Self { bytes: [0; HEADER_SIZE] };
        header.bytes[DATA_TYPE_CODE] = UNCOMPRESSED_RGB;
        header.bytes[BITS_PER_PIXEL] = 24;
        header.bytes[IMAGE_DESCRIPTOR] = TOP_TO_BOTTOM_ORDER;
        header.set_width(width);
        header.set_height(height);

        header
    }

    pub fn create_from_buffer(buffer: &[u8]) -> Self {
        let mut header = Self { bytes: [0; HEADER_SIZE] };

        for i in 0..18 {
            header.bytes[i] = buffer[i];
        }

        header
    }

    pub fn set_width(&mut self, width: u16) {
        self.set_u16_value(width, WIDTH_LO, WIDTH_HI);
    }

    pub fn set_height(&mut self, height: u16) {
        self.set_u16_value(height, HEIGHT_LO, HEIGHT_HI);
    }

    fn set_u16_value(&mut self, value: u16, lo: usize, hi: usize) {
        if value <= u8::MAX as u16 {
            self.bytes[lo] = value as u8;
            self.bytes[hi] = 0;
            return;
        }

        let bytes = value.to_be_bytes();
        self.bytes[hi] = bytes[0];
        self.bytes[lo] = bytes[1];
    }

    pub fn get_width(&mut self) -> u16 {
        u16::from_le_bytes([self.bytes[WIDTH_LO], self.bytes[WIDTH_HI]])
    }

    pub fn get_height(&mut self) -> u16 {
        u16::from_le_bytes([self.bytes[HEIGHT_LO], self.bytes[HEIGHT_HI]])
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn is_rle(&self) -> bool {
        self.bytes[DATA_TYPE_CODE] == 10 || self.bytes[DATA_TYPE_CODE] == 11
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_create() {
        let h = Header::create(10, 10);
        let b = h.to_bytes();

        assert_eq!(HEADER_SIZE, b.len(), "should be 18 bytes");
        assert_ne!(0, b[DATA_TYPE_CODE], "uncompressed byte set");
        assert_ne!(0, b[BITS_PER_PIXEL], "bits per pixel byte set");
        assert_ne!(0, b[IMAGE_DESCRIPTOR], "image descriptor byte set");
        assert_ne!(0, b[WIDTH_LO], "lower 256 - width low byte set");
        assert_ne!(0, b[HEIGHT_LO], "lower 256 - height byte set");

        let h = Header::create(257, 257);
        let b = h.to_bytes();
        assert_ne!(0, b[WIDTH_LO], "higher 256 - width low byte set");
        assert_ne!(0, b[WIDTH_HI], "higher 256 - width high byte set");
        assert_ne!(0, b[HEIGHT_LO], "higher 256 - height low byte set");
        assert_ne!(0, b[HEIGHT_HI], "higher 256 - height high byte set");
    }

    #[test]
    fn header_create_from_buffer() {
        let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let h = Header::create_from_buffer(&buf);
        let b = h.to_bytes();

        assert_eq!(HEADER_SIZE, b.len(), "should be 18 bytes");
        assert_eq!(&buf[..HEADER_SIZE], b, "should all bytes keep order");
    }

    #[test]
    fn header_set_width_height() {
        let mut h = Header::create(0, 0);
        let b = h.to_bytes();

        assert_eq!(0, b[WIDTH_LO], "init - width low byte should be 0");
        assert_eq!(0, b[WIDTH_HI], "init - width high byte should be 0");
        assert_eq!(0, b[HEIGHT_LO], "init - height low byte should be 0");
        assert_eq!(0, b[HEIGHT_HI], "init - height high byte should be 0");

        h.set_width(128);
        h.set_height(64);

        let b = h.to_bytes();
        assert_eq!(128, b[WIDTH_LO], "lower 256 - width low byte set");
        assert_eq!(0, b[WIDTH_HI], "lower 256 - width high byte set 0");
        assert_eq!(64, b[HEIGHT_LO], "lower 256 - height low byte set");
        assert_eq!(0, b[HEIGHT_HI], "lower 256 - height high byte set 0");

        h.set_width(300);
        h.set_height(800);

        let b = h.to_bytes();
        assert_eq!(44, b[WIDTH_LO], "higher 256 - width low byte set");
        assert_eq!(1, b[WIDTH_HI], "higher 256 - width high byte set");
        assert_eq!(32, b[HEIGHT_LO], "higher 256 - height low byte set");
        assert_eq!(3, b[HEIGHT_HI], "higher 256 - height high byte set");
    }

    #[test]
    fn header_get_width_height() {
        let mut h = Header::create(0, 0);

        assert_eq!(0, h.get_width(), "zero - get width");
        assert_eq!(0, h.get_height(), "zero - get height");

        h.set_width(10);
        h.set_height(20);

        assert_eq!(10, h.get_width(), "lower 256 - get width");
        assert_eq!(20, h.get_height(), "lower 256 - get height");

        h.set_width(1000);
        h.set_height(2000);

        assert_eq!(1000, h.get_width(), "higher 256 - get width");
        assert_eq!(2000, h.get_height(), "higher 256 - get height");
    }

    #[test]
    fn header_is_rle() {
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        buf[DATA_TYPE_CODE] = 10;
        let h = Header::create_from_buffer(&buf);
        assert!(h.is_rle());

        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        buf[DATA_TYPE_CODE] = 11;
        let h = Header::create_from_buffer(&buf);
        assert!(h.is_rle());

        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        buf[DATA_TYPE_CODE] = 2;
        let h = Header::create_from_buffer(&buf);
        assert!(!h.is_rle());

        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        buf[DATA_TYPE_CODE] = 3;
        let h = Header::create_from_buffer(&buf);
        assert!(!h.is_rle());
    }
}
