//const IDLENGTH: usize = 0;
//const COLORMAPTYPE: usize = 1;
const DATATYPECODE: usize = 2;
//const COLORMAPORIGIN_LO: usize = 3;
//const COLORMAPORIGIN_HI: usize = 4;
//const COLORMAPLENGTH_LO: usize = 5;
//const COLORMAPLENGTH_HI: usize = 6;
//const COLORMAPDEPTH: usize = 7;
//const XORIGIN_LO: usize = 8;
//const XORIGIN_HI: usize = 9;
//const YORIGIN_LO: usize = 10;
//const YORIGIN_HI: usize = 11;
const WIDTH_LO: usize = 12;
const WIDTH_HI: usize = 13;
const HEIGHT_LO: usize = 14;
const HEIGHT_HI: usize = 15;
const BITSPERPIXEL: usize = 16;
const IMAGEDESCRIPTOR: usize = 17;
const UNCOMPRESS_RGB: u8 = 2;
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
        header.bytes[DATATYPECODE] = UNCOMPRESS_RGB;
        header.bytes[BITSPERPIXEL] = 24;
        header.bytes[IMAGEDESCRIPTOR] = TOP_TO_BOTTOM_ORDER;
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
        if width < 256 {
            self.bytes[WIDTH_HI] = 0;
            self.bytes[WIDTH_LO] = width as u8;
        } else {
            let bytes = Self::split_u16_to_u8(width);
            self.bytes[WIDTH_HI] = bytes[0];
            self.bytes[WIDTH_LO] = bytes[1];
        }
    }

    pub fn set_height(&mut self, height: u16) {
        if height < 256 {
            self.bytes[HEIGHT_HI] = 0;
            self.bytes[HEIGHT_LO] = height as u8;
        } else {
            let bytes = Self::split_u16_to_u8(height);
            self.bytes[HEIGHT_HI] = bytes[0];
            self.bytes[HEIGHT_LO] = bytes[1];
        }
    }

    pub fn get_width(&mut self) -> u16 {
        Self::union_u8_to_u16(self.bytes[WIDTH_LO], self.bytes[WIDTH_HI])
    }

    pub fn get_height(&mut self) -> u16 {
        Self::union_u8_to_u16(self.bytes[HEIGHT_LO], self.bytes[HEIGHT_HI])
    }

    fn union_u8_to_u16(a: u8, b: u8) -> u16 {
        (b as u16) << 8 | a as u16
    }

    fn split_u16_to_u8(a: u16) -> [u8; 2] {
        a.to_be_bytes()
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn is_rle(&self) -> bool {
        self.bytes[DATATYPECODE] == 3 || self.bytes[DATATYPECODE] == 2
    }
}
