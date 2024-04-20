use std::fs::OpenOptions;
use std::io::Read;
use tga::tgacolor::Color;
use tga::tgaimage::Image;

#[test]
fn image_save_file() {
    let mut img = Image::create(3, 3);
    let _ = img.set_pixel(0, 2, &Color::create(10, 20, 30));
    let _ = img.set_pixel(2, 0, &Color::create(30, 20, 10));

    img.save("tests/data/save/actual.tga");

    let mut expected = OpenOptions::new()
        .read(true)
        .open("tests/data/save/example.tga").expect("can't open expected file");

    let mut actual = OpenOptions::new()
        .read(true)
        .open("tests/data/save/actual.tga").expect("can't open actual file");

    let mut expected_data: Vec<u8> = vec![];
    expected.read_to_end(&mut expected_data).expect("can't read expected file");

    let mut actual_data: Vec<u8> = vec![];
    actual.read_to_end(&mut actual_data).expect("can't read actual file");

    assert_eq!(expected_data, actual_data);
}

#[test]
fn image_read_uncompressed_file() {
    let mut img = Image::create_from_file("tests/data/read_uncompressed/example.tga");
    assert_eq!(9, img.size());
    assert_eq!([30, 20, 10], img.get_pixel(0, 2).to_bytes());
    assert_eq!([10, 20, 30], img.get_pixel(2, 0).to_bytes());
}

#[test]
fn image_read_compressed_file() {
    let mut img = Image::create_from_file("tests/data/read_compressed/example.tga");
    assert_eq!(9, img.size());
    assert_eq!([30, 20, 10], img.get_pixel(0, 2).to_bytes());
    assert_eq!([10, 20, 30], img.get_pixel(2, 0).to_bytes());
}