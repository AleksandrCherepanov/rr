use std::fs;

use render::{geometry::Geometry, vertex::Vertex};
use tga::{
    tgaimage::Image,
};
use crate::cli::utils;
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
        let color = utils::extract_color(&data, 2)?;
        println!("{:?}", color);

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
