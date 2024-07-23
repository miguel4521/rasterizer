use camera::Camera;
use minifb::Key;
use na::{Point3, Vector3};
use std::time::Instant;

mod camera;
pub mod model;
pub mod rasterizer;
mod window;

fn main() {
    let mut win = window::Window::new(800, 800);

    let mut model = model::Model::new("assets/african_head.obj").unwrap();
    model
        .load_texture("assets/african_head_diffuse.tga")
        .unwrap();
    win.rasterizer.set_texture(
        model.texture(),
        model.texture_width(),
        model.texture_height(),
    );

    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::FRAC_PI_3,
        800.0 / 800.0,
        0.1,
        100.0,
    );

    let mut last_time = Instant::now();
    let mut frame_count = 0;

    while win.is_open() && !win.is_key_down(Key::Escape) {
        frame_count += 1;
        let current_time = Instant::now();
        if current_time.duration_since(last_time).as_secs_f32() >= 1.0 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = current_time;
        }

        // Handle camera movement
        if win.is_key_down(Key::W) {
            camera.move_forward(0.1);
        }
        if win.is_key_down(Key::S) {
            camera.move_backward(0.1);
        }
        if win.is_key_down(Key::A) {
            camera.move_left(0.1);
        }
        if win.is_key_down(Key::D) {
            camera.move_right(0.1);
        }
        if win.is_key_down(Key::Left) {
            camera.rotate_yaw(0.05);
        }
        if win.is_key_down(Key::Right) {
            camera.rotate_yaw(-0.05);
        }
        if win.is_key_down(Key::Up) {
            camera.rotate_pitch(0.05);
        }
        if win.is_key_down(Key::Down) {
            camera.rotate_pitch(-0.05);
        }

        // Update camera view and projection matrices
        win.rasterizer
            .set_view_projection(camera.view_matrix(), camera.projection_matrix());

        win.rasterizer.clear();
        win.rasterizer.draw_wireframe(&model);

        win.update();
    }
}
