use camera::Camera;
use minifb::Key;
use pollster::block_on;
use std::time::Instant;

mod camera;
mod gpu;
mod raster_pass;
mod util;
mod window;

fn main() {
    let mut win = window::Window::new(800, 600);

    let mut last_time = Instant::now();
    let mut frame_count = 0;

    block_on(win.update());

    while win.is_open() && !win.is_key_down(Key::Escape) {
        block_on(win.update());

        frame_count += 1;
        let current_time = Instant::now();
        if current_time.duration_since(last_time).as_secs_f32() >= 1.0 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = current_time;
        }
    }
}
