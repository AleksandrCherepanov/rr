use std::fs::OpenOptions;
use std::io::Read;
use render::geometry::Geometry;
use render::model::Model;
use render::render::Render;
use tga::tgacolor::{Color, YELLOW};
use tga::tgaimage::Image;

//TODO rewrite code and test not using files
//as an example it's better to make a simple 100x100 model for example
#[test]
fn render() {
    let m = Model::open("tests/data/render/model.obj");
    let mut img = Image::create(800, 800);
    let mut geo = Geometry::create(&mut img);
    let mut render = Render::create(&mut geo);

    render.render(m, &vec![Color::create_from_bytes(YELLOW)]);
    img.flip_vertically();
    img.save("tests/data/render/actual.tga");

    let mut expected_file = OpenOptions::new()
        .read(true)
        .open("tests/data/render/expected.tga").expect("can't read expected file");

    let mut actual_file = OpenOptions::new()
        .read(true)
        .open("tests/data/render/actual.tga").expect("can't read actual file");


    let mut expected_data: Vec<u8> = vec![];
    expected_file.read_to_end(&mut expected_data).expect("can't read expected data");

    let mut actual_data: Vec<u8> = vec![];
    actual_file.read_to_end(&mut actual_data).expect("can't read actual data");
}

#[test]
fn render_filled_with_texture() {
    let m = Model::open("tests/data/render_filled/model.obj");
    let mut img = Image::create(800, 800);
    let mut geo = Geometry::create(&mut img);
    let mut render = Render::create(&mut geo);

    let texture = Image::create_from_file("tests/data/render_filled/texture.tga");

    render.render_filled(m, Some(texture));
    img.flip_vertically();
    img.save("tests/data/render_filled/actual_textured.tga");

    let mut expected_file = OpenOptions::new()
        .read(true)
        .open("tests/data/render_filled/expected_textured.tga").expect("can't read expected file");

    let mut actual_file = OpenOptions::new()
        .read(true)
        .open("tests/data/render_filled/actual_textured.tga").expect("can't read actual file");


    let mut expected_data: Vec<u8> = vec![];
    expected_file.read_to_end(&mut expected_data).expect("can't read expected data");

    let mut actual_data: Vec<u8> = vec![];
    actual_file.read_to_end(&mut actual_data).expect("can't read actual data");
}

#[test]
fn render_filled_with_color() {
    let m = Model::open("tests/data/render_filled/model.obj");
    let mut img = Image::create(800, 800);
    let mut geo = Geometry::create(&mut img);
    let mut render = Render::create(&mut geo);

    render.render_filled(m, None);
    img.flip_vertically();
    img.save("tests/data/render_filled/actual_filled.tga");

    let mut expected_file = OpenOptions::new()
        .read(true)
        .open("tests/data/render_filled/expected_filled.tga").expect("can't read expected file");

    let mut actual_file = OpenOptions::new()
        .read(true)
        .open("tests/data/render_filled/actual_filled.tga").expect("can't read actual file");


    let mut expected_data: Vec<u8> = vec![];
    expected_file.read_to_end(&mut expected_data).expect("can't read expected data");

    let mut actual_data: Vec<u8> = vec![];
    actual_file.read_to_end(&mut actual_data).expect("can't read actual data");

    assert_eq!(expected_data, actual_data);
}
