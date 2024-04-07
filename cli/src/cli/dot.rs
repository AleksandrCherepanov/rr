use std::fs;

use render::{geometry::Geometry, vertex::Vertex};
use tga::{
    tgacolor::{Color, AVAILABLE_COLORS, DEFAULT_COLOR},
    tgaimage::Image,
};

use crate::DotsArgs;

pub fn cli_dot(args: &DotsArgs) -> Result<(), String> {
    let source = &args.source;
    let target = &args.target;
    let mut image = Image::create(args.width.unwrap_or(800), args.height.unwrap_or(800));

    let source = fs::read_to_string(&source).expect(&format!("File: {} not found.", source));
    let lines = source.split("\n");
    for l in lines {
        let data: Vec<&str> = l.trim().split(" ").collect();
        let vertex = extract_vertex(&data)?;
        let color = extract_color(&data)?;

        let geometry = Geometry::create(&mut image);
        geometry
            .image
            .set_pixel(vertex.x as isize, vertex.y as isize, &color);
    }

    image.save(&target);
    Ok(())
}

fn extract_vertex(data: &Vec<&str>) -> Result<Vertex, String> {
    check_coordinates(&data)?;
    Ok(Vertex::create_from_string(
        &format!("{} {}", data[0], data[1])[..],
    ))
}

fn check_coordinates(data: &Vec<&str>) -> Result<(), String> {
    if data.len() < 2 {
        return Err(format!(
            "The format should be X Y Color, {} given",
            data.join(" ")
        ));
    }

    Ok(())
}

fn check_color(color: &str) -> Result<(), String> {
    if !AVAILABLE_COLORS.contains_key(color) {
        let colors: Vec<&str> = AVAILABLE_COLORS.keys().cloned().collect();
        return Err(format!("The color should be on of: {:?}", colors));
    }

    Ok(())
}

fn extract_color(data: &Vec<&str>) -> Result<Color, String> {
    if data.len() <= 2 {
        return Ok(Color::create(
            DEFAULT_COLOR[0],
            DEFAULT_COLOR[1],
            DEFAULT_COLOR[2],
        ));
    }

    check_color(data[2])?;

    let color = AVAILABLE_COLORS.get(data[2]).unwrap();
    Ok(Color::create(color[0], color[1], color[2]))
}
