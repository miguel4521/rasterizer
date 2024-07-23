use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use image::GenericImageView;
use {na::Vector2, na::Vector3};

pub struct Model {
    verts: Vec<Vector3<f32>>,
    faces: Vec<Vec<(usize, usize, usize)>>,
    texcoords: Vec<Vector2<f32>>,
    texture: Option<Vec<u32>>,
    texture_width: usize,
    texture_height: usize,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Model> {
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut texcoords = Vec::new();

        if let Ok(file) = File::open(filename) {
            for line in io::BufReader::new(file).lines() {
                let line = line?;
                if line.starts_with("v ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let v = Vector3::new(
                        f32::from_str(parts[1]).unwrap(),
                        f32::from_str(parts[2]).unwrap(),
                        f32::from_str(parts[3]).unwrap(),
                    );
                    verts.push(v);
                } else if line.starts_with("vt ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let vt = Vector2::new(
                        f32::from_str(parts[1]).unwrap(),
                        f32::from_str(parts[2]).unwrap(),
                    );
                    texcoords.push(vt);
                } else if line.starts_with("f ") {
                    let mut face = Vec::new();
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for part in &parts[1..] {
                        let indices: Vec<&str> = part.split('/').collect();
                        let vertex_idx = usize::from_str(indices[0]).unwrap() - 1;
                        let texcoord_idx = usize::from_str(indices[1]).unwrap() - 1;
                        let normal_idx = usize::from_str(indices[2]).unwrap() - 1;
                        face.push((vertex_idx, texcoord_idx, normal_idx));
                    }
                    faces.push(face);
                }
            }
        }

        Ok(Model {
            verts,
            faces,
            texcoords,
            texture: None,
            texture_width: 0,
            texture_height: 0,
        })
    }

    pub fn load_texture(&mut self, texture_filename: &str) -> io::Result<()> {
        let img =
            image::open(texture_filename).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        self.texture_width = img.width() as usize;
        self.texture_height = img.height() as usize;

        let mut texture_data = Vec::with_capacity(self.texture_width * self.texture_height);

        for pixel in img.pixels() {
            let (_, _, rgb) = pixel;
            let color = (rgb[0] as u32) << 16 | (rgb[1] as u32) << 8 | (rgb[2] as u32);
            texture_data.push(color);
        }

        self.texture = Some(texture_data);

        Ok(())
    }

    pub fn texcoord(&self, idx: usize) -> Vector2<f32> {
        self.texcoords[idx]
    }

    pub fn texture(&self) -> Option<&Vec<u32>> {
        self.texture.as_ref()
    }

    pub fn texture_width(&self) -> usize {
        self.texture_width
    }

    pub fn texture_height(&self) -> usize {
        self.texture_height
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn face(&self, idx: usize) -> &Vec<(usize, usize, usize)> {
        &self.faces[idx]
    }

    pub fn vert(&self, i: usize) -> Vector3<f32> {
        self.verts[i]
    }
}
