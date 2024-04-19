use std::io::Read;
use std::{self, io::Write};
use std::fs::OpenOptions;
use crate::tgacolor::*;
use crate::tgaheader::*;

pub struct Image {
    canva: Vec<Vec<Vec<u8>>>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn create(width: usize, height: usize) -> Self {
        Self {
            canva: vec![vec![vec![0; RGB_LEN as usize]; width]; height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: &Color) -> bool {
        if self.canva.len() == 0 || x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return false;
        }

        for i in 0..RGB_LEN {
            self.canva[y as usize][x as usize][i] = color.get_bytes()[i];
        }

        true
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> Color {
        if self.canva.len() == 0 || x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return Color::create(0, 0, 0);
        }

        let pixel = &self.canva[y as usize][x as usize];
        Color::create(pixel[2], pixel[1], pixel[0])
    }

    pub fn size(&mut self) -> usize {
        return self.width * self.height;
    }

    pub fn flip_vertically(&mut self) -> bool {
        if self.canva.len() == 0 {
            return false;
        }

        let half: usize = self.height / 2;

        for i in 0..half {
            let l1 = &self.canva[i];
            let l2 = &self.canva[self.height - 1 - i];
            let line = Vec::from_iter(l1.iter().cloned());
            self.canva[i] = Vec::from_iter(l2.iter().cloned());
            self.canva[self.height - 1 - i] = Vec::from_iter(line.iter().cloned());
        }

        true
    }

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

        let _ = file.write_all(tga_header.get_bytes());
        for i in 0..self.canva.len() {
            let mut line = vec![0 as u8; self.canva[i].len() * RGB_LEN];
            let mut p = 0;
            for j in 0..self.canva[i].len() {
                for k in 0..self.canva[i][j].len() {
                    line[p] = self.canva[i][j][k];
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

        if tga_header.is_rle() {
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
