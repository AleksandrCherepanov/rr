use std::io::Read;
use std::{self, io::Write};
use std::fs::OpenOptions;
use crate::tgacolor::*;
use crate::tgaheader::*;

#[derive(Debug)]
pub struct Image {
    canvas: Vec<Vec<Vec<u8>>>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn create(width: usize, height: usize) -> Self {
        Self {
            canvas: vec![vec![vec![0; RGB_LEN]; width]; height],
            width,
            height,
        }
    }

    //TODO make this values usize as the minimum value is (0, 0)
    pub fn set_pixel(&mut self, x: isize, y: isize, color: &Color) -> bool {
        if self.canvas.len() == 0 || x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return false;
        }

        //TODO strange inversion here with pos, needs to be fixed
        for i in 0..RGB_LEN {
            self.canvas[y as usize][x as usize][i] = color.to_bytes()[i];
        }

        true
    }

    //TODO it's better to return error, who knows
    pub fn get_pixel(&self, x: isize, y: isize) -> Color {
        if self.canvas.len() == 0 || x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return Color::create(0, 0, 0);
        }

        let pixel = &self.canvas[y as usize][x as usize];
        Color::create(pixel[2], pixel[1], pixel[0])
    }

    pub fn size(&mut self) -> usize {
        return self.width * self.height;
    }

    pub fn flip_vertically(&mut self) -> bool {
        if self.canvas.len() == 0 {
            return false;
        }

        let half: usize = self.height / 2;

        for i in 0..half {
            let l1 = &self.canvas[i];
            let l2 = &self.canvas[self.height - 1 - i];
            let line = Vec::from_iter(l1.iter().cloned());
            self.canvas[i] = Vec::from_iter(l2.iter().cloned());
            self.canvas[self.height - 1 - i] = Vec::from_iter(line.iter().cloned());
        }

        true
    }

    //TODO make two separate mod with writing/reading tga file
    pub fn save(&self, filename: &str) -> bool {
        let developer_area_ref: [u8; 4] = [0, 0, 0, 0];
        let extension_area_ref: [u8; 4] = [0, 0, 0, 0];
        let footer = "TRUEVISION-XFILE.\0";

        let tga_header = Header::create(self.width as u16, self.height as u16);

        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .expect("can't open file");

        let _ = file.write_all(tga_header.to_bytes());
        for i in 0..self.canvas.len() {
            let mut line = vec![0 as u8; self.canvas[i].len() * RGB_LEN];
            let mut p = 0;
            for j in 0..self.canvas[i].len() {
                for k in 0..self.canvas[i][j].len() {
                    line[p] = self.canvas[i][j][k];
                    p += 1;
                }
            }
            let _ = file.write_all(&line[..]);
        }
        let _ = file.write_all(&developer_area_ref[..]);
        let _ = file.write_all(&extension_area_ref[..]);
        let _ = file.write_all(&footer.as_bytes());
        true
    }

    pub fn create_from_file(filename: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .open(filename)
            .expect("can't open file");

        let mut buffer: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut buffer);

        let mut tga_header = Header::create_from_buffer(&buffer[0..HEADER_SIZE]);
        let mut image = Image::create(
            tga_header.get_width() as usize,
            tga_header.get_height() as usize,
        );

        let mut row: isize = 0;
        let mut col: isize = 0;
        let mut i = HEADER_SIZE;
        let mut count = 0;

        if !tga_header.is_rle() {
            let mut col: usize = 0;
            let mut row: usize = 0;
            while col < image.height {
                image.set_pixel(row as isize, col as isize, &Color::create(buffer[i + 2], buffer[i + 1], buffer[i]));
                count += RGB_LEN;
                row += 1;
                if count == image.width * RGB_LEN {
                    col += 1;
                    count = 0;
                    row = 0;
                }
                i += RGB_LEN;
            }
        } else {
            while count < image.size() {
                let mut package_size = buffer[i];
                let mut pixels_count = RGB_LEN;
                let mut raw_pixels = 0;

                i += 1;

                if package_size < 128 {
                    package_size += 1;
                    pixels_count = 0;
                    raw_pixels = RGB_LEN;
                } else {
                    package_size -= 127;
                }

                for _ in 0..package_size as usize {
                    image.set_pixel(col, row, &Color::create(buffer[i + 2], buffer[i + 1], buffer[i]));
                    (col, row) = Self::get_next_cell(col, row, image.width);
                    i += raw_pixels;
                }
                count += package_size as usize;
                i += pixels_count;
            }
        }

        image
    }

    fn get_next_cell(col: isize, row: isize, width: usize) -> (isize, isize) {
        let mut new_col = col + 1;
        let mut new_row = row;
        if new_col == width as isize {
            new_col = 0;
            new_row += 1;
        }

        (new_col, new_row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_create() {
        let img = Image::create(5, 5);

        for i in 0..5 {
            for j in 0..5 {
                let c = img.get_pixel(i, j);
                assert_eq!([0, 0, 0], c.to_bytes());
            }
        }
    }

    #[test]
    fn image_set_get_pixel() {
        let mut img = Image::create(3, 3);
        let r = img.set_pixel(0, 2, &Color::create(10, 20, 30));
        assert!(r, "top-right - should be set");
        assert_eq!([30, 20, 10], img.get_pixel(0, 2).to_bytes());

        let r = img.set_pixel(2, 0, &Color::create(30, 20, 10));
        assert!(r, "bottom-left - should be set");
        assert_eq!([10, 20, 30], img.get_pixel(2, 0).to_bytes());

        let r = img.set_pixel(-10, 0, &Color::create(30, 20, 10));
        assert!(!r, "x negative - won't be set");

        let r = img.set_pixel(0, -10, &Color::create(30, 20, 10));
        assert!(!r, "y negative - won't be set");

        let r = img.set_pixel(10, 0, &Color::create(30, 20, 10));
        assert!(!r, "x out of the border - won't be set");

        let r = img.set_pixel(0, 10, &Color::create(30, 20, 10));
        assert!(!r, "y out of the border - won't be set");

        let r = img.get_pixel(-10, 0);
        assert_eq!([0, 0, 0], r.to_bytes(), "x negative - won't be get");

        let r = img.get_pixel(0, -10);
        assert_eq!([0, 0, 0], r.to_bytes(), "y negative - won't be get");

        let r = img.get_pixel(10, 0);
        assert_eq!([0, 0, 0], r.to_bytes(), "x out of the border - won't be get");

        let r = img.get_pixel(0, 10);
        assert_eq!([0, 0, 0], r.to_bytes(), "y out of the border - won't be set");
    }

    #[test]
    fn image_size() {
        let mut img = Image::create(3, 3);
        assert_eq!(9, img.size());
    }

    #[test]
    fn image_flip_vertically() {
        let mut img = Image::create(3, 3);
        let _ = img.set_pixel(0, 2, &Color::create(10, 20, 30));
        let _ = img.set_pixel(2, 0, &Color::create(30, 20, 10));

        let r = img.flip_vertically();
        assert!(r, "should be flipped vertically");
        assert_eq!([0, 0, 0], img.get_pixel(0, 2).to_bytes(), "top-right - empty after flip");
        assert_eq!([30, 20, 10], img.get_pixel(0, 0).to_bytes(), "top-left - should be set");
        assert_eq!([10, 20, 30], img.get_pixel(2, 2).to_bytes(), "bottom-right - should be set");

        let mut img = Image::create(0, 0);
        let r = img.flip_vertically();
        assert!(!r, "empty image - should not be flipped vertically");
    }
}
