use render::{geometry::Geometry, model::Model, render::Render};
use tga::tgaimage::Image;

use crate::ModelArgs;

pub fn cli_model(args: &ModelArgs) -> Result<(), String> {
    let source = &args.source;
    let target = &args.target;
    let texture = &args.texture;

    let mut image = Image::create(args.width.unwrap_or(800), args.height.unwrap_or(800));
    let mut texture_image: Option<Image> = None;
    if texture.is_some() {
        texture_image = Some(Image::create_from_file(&texture.as_ref().unwrap()[..]));
    }

    let mut geometry = Geometry::create(&mut image);
    let mut render = Render::create(&mut geometry);

    let model = Model::open(&source);

    render.render_filled(model, texture_image);
    image.flip_vertically();
    image.save(&target);
    Ok(())
}
