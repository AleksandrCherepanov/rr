use render::{
    geometry::Geometry,
    model::Model,
    render::Render,
};
use tga::{
    tgacolor::{
        Color as TgaColor, BLUE, DARK_BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW,
    },
    tgaimage::Image,
};

use crate::{Color, SkeletonArgs};

pub fn cli_skeleton(args: &SkeletonArgs) -> Result<(), String> {
    let source = &args.source;
    let target = &args.target;

    let mut image = Image::create(args.width.unwrap_or(800), args.height.unwrap_or(800));
    let mut geometry = Geometry::create(&mut image);
    let mut render = Render::create(&mut geometry);

    let model = Model::open(&source);
    let color: [u8; 3];

    match &args.color {
        Color::Red => color = RED,
        Color::Orange => color = ORANGE,
        Color::Yellow => color = YELLOW,
        Color::Green => color = GREEN,
        Color::Blue => color = BLUE,
        Color::DarkBlue => color = DARK_BLUE,
        Color::Purple => color = PURPLE,
        Color::White => color = WHITE,
    }

    let color = TgaColor::create(color[0], color[1], color[2]);
    render.render(model, &vec![color]);
    image.flip_vertically();
    image.save(&target);
    Ok(())
}
