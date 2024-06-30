use vecs::vec2::Vec2i;

pub mod model;
pub mod rasterizer;
pub mod vecs;
mod window;

fn main() {
    let mut win = window::Window::new(800, 800);

    let model = model::Model::new("assets/african_head.obj").unwrap();

    while win.is_open() && !win.is_key_down(minifb::Key::Escape) {
        win.rasterizer.clear();

        win.rasterizer.draw_wireframe(&model);

        win.update();
    }
}
