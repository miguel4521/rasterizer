mod window;
pub mod model;

fn main() {
    let mut win = window::Window::new(800, 800);

    let model = model::Model::new("assets/african_head.obj").unwrap();

    while win.is_open() && !win.is_key_down(minifb::Key::Escape) {
        win.clear();

        win.render_wireframe(&model);

        win.update();
    }
}