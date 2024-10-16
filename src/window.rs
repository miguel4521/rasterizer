use glam::{vec3, Vec3};
use pollster::block_on;

use crate::{camera::Camera, gpu};

pub struct Window {
    window: minifb::Window,
    pub gpu: gpu::GPU,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let camera = Camera::new(
            0.5,
            36.0,
            0.0,
            vec3(0.0, -0.5, 0.0),
            width as f32 / height as f32,
        );

        let gpu = block_on(gpu::GPU::new(width as u32, height as u32, camera));
        let window = minifb::Window::new(
            "Minimal Renderer - ESC to exit",
            width,
            height,
            minifb::WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Window { gpu, window }
    }

    pub async fn update(&mut self) {
        let buffer = self.gpu.get_output_buffer_data().await;

        // Update the window with the buffer
        self.window
            .update_with_buffer(&buffer, self.gpu.width as usize, self.gpu.height as usize)
            .expect("Failed to update window");
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }
}
