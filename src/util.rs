use std::io::Read;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[allow(clippy::iter_nth_zero)]
pub fn process_gltf_model() -> Vec<Vertex> {
    let (model, buffers, _) = {
        let bytes = include_bytes!("../assets/suzanne.glb");
        gltf::import_slice(bytes).unwrap()
    };
    let mesh = model.meshes().nth(0).unwrap();
    let primitives = mesh.primitives().nth(0).unwrap();
    let reader = primitives.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions = reader.read_positions().unwrap().collect::<Vec<_>>();
    reader
        .read_indices()
        .unwrap()
        .into_u32()
        .map(|i| Vertex::from(positions[i as usize]))
        .collect()
}

#[allow(dead_code)]
pub fn process_obj_model(file: impl Read) -> Vec<Vertex> {
    obj::ObjData::load_buf(file)
        .unwrap()
        .position
        .iter()
        .cloned()
        .map(Vertex::from)
        .collect()
}

pub(crate) const WORKGROUP_SIZE: u32 = 256;
pub(crate) const fn dispatch_size(len: u32) -> u32 {
    let subgroup_size = WORKGROUP_SIZE;
    (len + subgroup_size - 1) / subgroup_size
}

pub fn get_output_buffer_size(width: u32, height: u32) -> u64 {
    use std::mem::size_of;

    let pixel_size = size_of::<u32>() as u64;
    let (width, height) = (width as u64, height as u64);
    width * height * pixel_size
}

pub fn create_output_buffer(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Buffer {
    let pixel_buffer_size = (width as usize * height as usize * std::mem::size_of::<u32>()) as u64; // Calculate the buffer size needed

    // Create a new buffer suitable for storing u32 pixel data and for being bound to a storage buffer
    let pixel_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Pixel Buffer"),
        size: pixel_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    pixel_buffer
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(crate) struct Uniform {
    screen_width: f32,
    screen_height: f32,
}

impl Uniform {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            screen_width,
            screen_height,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    v: [f32; 3],
}

impl Vertex {
    pub const SIZE: u64 = std::mem::size_of::<Self>() as _;
    pub const ATTR: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { v: [x, y, z] }
    }
}

macro_rules! v {
    ($x:expr, $y:expr, $z:expr) => {
        Vertex::new($x, $y, $z)
    };
}
pub(crate) use v;

impl From<[f32; 3]> for Vertex {
    fn from(v: [f32; 3]) -> Self {
        v!(v[0], v[1], v[2])
    }
}

#[allow(dead_code)]
pub const TRIG: [Vertex; 3] = [v!(0.0, 0.5, 0.0), v!(-0.5, 0.0, 0.0), v!(0.5, 0.0, 0.0)];
