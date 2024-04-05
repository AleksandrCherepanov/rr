use std::{fs::OpenOptions, io::{BufRead, BufReader}};
use crate::vertex::*;

pub struct Face {
    pub vertices: Vec<usize>,
    pub textures: Vec<usize>,
    pub normals: Vec<usize>,
}

impl Face {
    fn create_from_string(data: &str) -> Self {
        let raw_faces: Vec<&str> = data.split_whitespace().collect();
        
        let mut vertices: Vec<usize> = vec![];
        let mut textures: Vec<usize> = vec![];
        let mut normals: Vec<usize> = vec![];
        
        for raw_face in raw_faces.iter() {
            let nums: Vec<&str> = raw_face.split_terminator('/').collect();
            let mut nums = nums.iter().map(|el| if el.is_empty() { 1 } else {el.parse::<usize>().expect("can't parse line to face")});

            vertices.push(nums.next().unwrap() - 1);
            textures.push(nums.next().unwrap() - 1);
            normals.push(nums.next().unwrap() - 1);
        } 

        Self {
            vertices,
            textures,
            normals,
        } 
    }
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub textures: Vec<Vertex>
}

impl Model {
    pub fn open(filename: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(filename)
            .expect(&(String::from("can't open model file: ") + filename));

        let reader = BufReader::new(file).lines().flatten();
        
        let mut vertices: Vec<Vertex> = vec![];
        let mut faces: Vec<Face> = vec![];
        let mut textures: Vec<Vertex> = vec![];
        for line in reader {
            if line.starts_with("v ") {
                let vrtx = Vertex::create_from_string(&line[2..]);
                vertices.push(vrtx);
            }

            if line.starts_with("vt ") {
                let txtr = Vertex::create_from_string(&line[3..]);
                textures.push(txtr);
            }

            if line.starts_with("f ") {
                let face = Face::create_from_string(&line[2..]);
                faces.push(face);
            }
        }

        Self {
            vertices,
            faces,
            textures
        }
    }
}