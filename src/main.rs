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

        win.rasterizer.draw_triangle(
            &Vec2i { x: 10, y: 70 },
            &Vec2i { x: 50, y: 160 },
            &Vec2i { x: 70, y: 80 },
            &0xff_ff_ff,
        );

        win.rasterizer.fill_triangle(
            &Vec2i { x: 180, y: 50 },
            &Vec2i { x: 150, y: 1 },
            &Vec2i { x: 70, y: 180 },
            &0xff_ff_ff,
        );

        win.update();
    }
}
