use std::fs;

use render::{geometry::Geometry, vertex::Vertex};
use tga::{
    tgaimage::Image,
};
use crate::cli::utils;

use crate::LinesArgs;

pub fn cli_line(args: &LinesArgs) -> Result<(), String> {
    let source = &args.source;
    let target = &args.target;
    let mut image = Image::create(args.width.unwrap_or(800), args.height.unwrap_or(800));

    let source = fs::read_to_string(&source).expect(&format!("File: {} not found.", source));
    let lines = source.split("\n");
    for l in lines {
        let data: Vec<&str> = l.trim().split(" ").collect();
        let (mut a, mut b) = extract_vertex(&data)?;
        let color = utils::extract_color(&data, 4)?;

        let mut geometry = Geometry::create(&mut image);
        geometry.line(&mut a, &mut b, &color);
    }

    image.save(&target);
    Ok(())
}

fn extract_vertex(data: &Vec<&str>) -> Result<(Vertex, Vertex), String> {
    check_coordinates(&data)?;
    let a = Vertex::create_from_string(&format!("{} {}", data[0], data[1])[..]);
    let b = Vertex::create_from_string(&format!("{} {}", data[2], data[3])[..]);
    Ok((a, b))
}

fn check_coordinates(data: &Vec<&str>) -> Result<(), String> {
    if data.len() < 4 {
        return Err(format!(
            "The format should be X1 Y1 X2 Y2 Color, {} given",
            data.join(" ")
        ));
    }

    Ok(())
}
