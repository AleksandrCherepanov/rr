use std::fs;

use render::{color::Color as RenderColor, geometry::Geometry, vertex::Vertex};
use tga::{
    tgacolor::{Color, AVAILABLE_COLORS, DEFAULT_COLOR},
    tgaimage::Image,
};

use crate::TrianglesArgs;

pub fn cli_triangle(args: &TrianglesArgs) -> Result<(), String> {
    let source = &args.source;
    let target = &args.target;
    let mut image = Image::create(args.width.unwrap_or(800), args.height.unwrap_or(800));

    let source = fs::read_to_string(&source).expect(&format!("File: {} not found.", source));
    let lines = source.split("\n");
    for l in lines {
        let data: Vec<&str> = l.trim().split(" ").collect();
        let (mut a, mut b, mut c) = extract_vertex(&data)?;
        let color = extract_color(&data)?;

        let mut geometry = Geometry::create(&mut image);
        if args.filled {
            let mut color_resolver = RenderColor {
                color
            };
            geometry.polygon(&mut a, &mut b, &mut c, &mut color_resolver, 1.0);
        } else {
            geometry.triangle(&mut a, &mut b, &mut c, &color);
        }
    }

    image.save(&target);
    Ok(())
}

fn extract_vertex(data: &Vec<&str>) -> Result<(Vertex, Vertex, Vertex), String> {
    check_coordinates(&data)?;
    let a = Vertex::create_from_string(&format!("{} {}", data[0], data[1])[..]);
    let b = Vertex::create_from_string(&format!("{} {}", data[2], data[3])[..]);
    let c = Vertex::create_from_string(&format!("{} {}", data[4], data[5])[..]);
    Ok((a, b, c))
}

fn check_coordinates(data: &Vec<&str>) -> Result<(), String> {
    if data.len() < 6 {
        return Err(format!(
            "The format should be X1 Y1 X2 Y2 X3 X3 Color, {} given",
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
    if data.len() <= 6 {
        return Ok(Color::create(
            DEFAULT_COLOR[0],
            DEFAULT_COLOR[1],
            DEFAULT_COLOR[2],
        ));
    }

    check_color(data[6])?;

    let color = AVAILABLE_COLORS.get(data[6]).unwrap();
    Ok(Color::create(color[0], color[1], color[2]))
}
