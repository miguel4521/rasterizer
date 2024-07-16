pub mod model;
pub mod rasterizer;
pub mod vecs;
mod window;

fn main() {
    let mut win = window::Window::new(800, 800);

    let mut model = model::Model::new("assets/african_head.obj").unwrap();
    model.load_texture("assets/african_head_diffuse.tga").unwrap();
    win.rasterizer.set_texture(model.texture(), model.texture_width(), model.texture_height());

    while win.is_open() && !win.is_key_down(minifb::Key::Escape) {
        win.rasterizer.clear();

        win.rasterizer.draw_wireframe(&model);

        win.update();
    }
}
