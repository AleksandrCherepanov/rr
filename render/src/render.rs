use rand::Rng;

use crate::vertex::Vertex;
use crate::light::Light;
use crate::geometry::*;
use crate::model::*;
use tga::tgacolor::*;
use tga::tgaimage::*;

pub struct Render<'a> {
    geometry: &'a mut Geometry<'a>
}

impl<'a> Render<'a> {
    pub fn create(geometry: &'a mut Geometry<'a>) -> Self {
        Self {
            geometry
        }
    } 

    pub fn render(&mut self, model: Model, color: &Vec<Color>) {
        for face in model.faces {
            for i in 0..3 {
                let clr = rand::thread_rng().gen_range(0..color.len());
                let vrtx1 = &model.vertices[face.vertices[i]];
                let vrtx2 = &model.vertices[face.vertices[(i + 1) % 3]];

                let x1 = (vrtx1.x + 1.0) * self.geometry.image.width as f64 / 2.0;
                let y1 = (vrtx1.y + 1.0) * self.geometry.image.height as f64 / 2.0;
                let x2 = (vrtx2.x + 1.0) * self.geometry.image.width as f64 / 2.0;
                let y2 = (vrtx2.y + 1.0) * self.geometry.image.height as f64 / 2.0;

                let mut a = Vertex::create(x1, y1, 0.0);
                let mut b = Vertex::create(x2, y2, 0.0);

                self.geometry.line(&mut a, &mut b, &color[clr]);
            }
        }
    }
    
    pub fn render_filled(&mut self, model: Model, texture: Image) {
        let light_direction = Vertex::create(0.0, 0.0, -1.0);
        let mut zbuff = vec![vec![i32::MIN; self.geometry.image.width as usize]; self.geometry.image.height as usize];
        for face in model.faces {
            let mut screen_coords: Vec<Vertex> = Vec::new();
            let mut world_coord: Vec<&Vertex> = Vec::new();
            let mut texture_coord: Vec<Vertex> = Vec::new();

            for i in 0..3 {
                let vrtx = &model.vertices[face.vertices[i]];

                let x = (vrtx.x + 1.0) * self.geometry.image.width as f64 / 2.0;
                let y = (vrtx.y + 1.0) * self.geometry.image.height as f64 / 2.0;
                let z = (vrtx.z + 1.0) * 255.0 / 2.0;
                
                let tx = &model.textures[face.textures[i]];
                let txx = tx.x * texture.width as f64;
                let txy = tx.y * texture.height as f64;

                screen_coords.push(Vertex::create(x, y, z));
                world_coord.push(vrtx);
                texture_coord.push(Vertex::create(txx, txy, 0.0));
            }
            
            let light = Light::calculate_light(world_coord[2], world_coord[1], world_coord[0]);
            let intensity = light.mul_scalar(&light_direction);

            let mut v1 = screen_coords[0].copy(); 
            let mut v2 = screen_coords[1].copy(); 
            let mut v3 = screen_coords[2].copy(); 

            if intensity > 0.0 {
                self.geometry.polygon(&mut v1, &mut v2, &mut v3, &mut zbuff, &texture_coord, &texture, intensity);
            }
        }
    }
}
