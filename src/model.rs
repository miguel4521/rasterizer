use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use crate::vecs::vec3::Vec3f;

pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Model> {
        let mut verts = Vec::new();
        let mut faces = Vec::new();

        if let Ok(file) = File::open(filename) {
            for line in io::BufReader::new(file).lines() {
                let line = line?;
                if line.starts_with("v ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let v = Vec3f::new(
                        f32::from_str(parts[1]).unwrap(),
                        f32::from_str(parts[2]).unwrap(),
                        f32::from_str(parts[3]).unwrap(),
                    );
                    verts.push(v);
                } else if line.starts_with("f ") {
                    let mut face = Vec::new();
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for part in &parts[1..] {
                        let indices: Vec<&str> = part.split('/').collect();
                        let idx = usize::from_str(indices[0]).unwrap() - 1;
                        face.push(idx);
                    }
                    faces.push(face);
                }
            }
        }

        eprintln!("# v# {} f# {}", verts.len(), faces.len());

        Ok(Model { verts, faces })
    }

    fn nverts(&self) -> usize {
        self.verts.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn face(&self, idx: usize) -> &Vec<usize> {
        &self.faces[idx]
    }

    pub fn vert(&self, i: usize) -> Vec3f {
        self.verts[i]
    }
}