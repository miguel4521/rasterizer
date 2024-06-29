use crate::rasterizer;

pub struct Window {
    window: minifb::Window,
    pub rasterizer: rasterizer::Rasterizer,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let rasterizer = rasterizer::Rasterizer::new(width, height);
        let window = minifb::Window::new(
            "Minimal Renderer - ESC to exit",
            width,
            height,
            minifb::WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Window {
            rasterizer,
            window,
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.rasterizer.buffer, self.rasterizer.width, self.rasterizer.height)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }
}
