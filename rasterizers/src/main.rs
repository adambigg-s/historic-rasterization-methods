mod barycentric;
mod scanline;
mod shared;

use minifb::Scale;
use minifb::Window;
use minifb::WindowOptions;

use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector3;
use toolbox::vec3;
use toolbox::vector;

use shared::pack_color;
use shared::should_exit;
use shared::Triangle;
use shared::Vertex;

const WIDTH: usize = 1520;
const HEIGHT: usize = 855;

fn main() {
    let window_handle = Window::new(
        "historic rasterizers",
        WIDTH,
        HEIGHT,
        WindowOptions { resize: false, scale: Scale::X1, ..Default::default() },
    );
    let mut window = window_handle.expect("failed to open window");
    window.set_target_fps(9999999);

    let mut buffer = Buffer2::new(WIDTH, HEIGHT, vector!(0.1, 0.1, 0.15));

    #[rustfmt::skip]
    let mut triangle = Triangle {
        vertices: [
            Vertex { pos: vector!(-0.3, -0.5, 0), col: vector!(1  , 0.7, 0  ) },
            Vertex { pos: vector!(0.3 , -0.5, 0), col: vector!(0  , 1  , 0.7) },
            Vertex { pos: vector!(0   , 0.5 , 0), col: vector!(0.7, 0  , 1  ) },
        ],
        pos: vector!(0, 0, 10),
        rot: vector!(0, 0, 0),
        scale: 5.5,
    };

    while !should_exit(&window) {
        triangle.rot = triangle.rot + vector!(0.01, 0.02, 0.001);

        buffer.clear();
        barycentric::render(&mut buffer, &triangle);
        scanline::render(&mut buffer, &triangle);

        window
            .update_with_buffer(
                &buffer.data.iter().map(pack_color).collect::<Vec<u32>>(),
                buffer.width,
                buffer.height,
            )
            .expect("failed to update buffer");
    }
}
