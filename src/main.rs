use vecs::vec2::Vec2f;

pub mod model;
pub mod rasterizer;
mod window;
pub mod vecs;

fn main() {
    let mut win = window::Window::new(800, 800);

    let model = model::Model::new("assets/african_head.obj").unwrap();

    while win.is_open() && !win.is_key_down(minifb::Key::Escape) {
        win.rasterizer.clear();

        win.rasterizer.draw_wireframe(&model);

        win.rasterizer.draw_triangle(
            &Vec2f { x: 10, y: 70 },
            &Vec2f { x: 50, y: 160 },
            &Vec2f { x: 70, y: 80 },
            &0xff_ff_ff,
        );

        win.rasterizer.fill_triangle(
            &Vec2f { x: 180, y: 50 },
            &Vec2f { x: 150, y: 1 },
            &Vec2f { x: 70, y: 180 },
            &0xff_ff_ff,
        );

        win.update();
    }
}
