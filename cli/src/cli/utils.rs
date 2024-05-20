use tga::tgacolor::{AVAILABLE_COLORS, Color, DEFAULT_COLOR};

fn check_color(color: &str) -> Result<(), String> {
    if !AVAILABLE_COLORS.contains_key(color) {
        let colors: Vec<&str> = AVAILABLE_COLORS.keys().cloned().collect();
        return Err(format!("The color should be on of: {:?}", colors));
    }

    Ok(())
}

pub fn extract_color(data: &Vec<&str>, arg_pos: usize) -> Result<Color, String> {
    if data.len() <= arg_pos {
        return Ok(Color::create(
            DEFAULT_COLOR[0],
            DEFAULT_COLOR[1],
            DEFAULT_COLOR[2],
        ));
    }

    check_color(data[arg_pos])?;

    let color = AVAILABLE_COLORS.get(data[arg_pos]).unwrap();
    Ok(Color::create(color[0], color[1], color[2]))
}
