mod barycentric;
mod raytraced;
mod scanline;
mod shared;

use minifb::Key;
use minifb::Scale;
use minifb::Window;
use minifb::WindowOptions;

use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector3;
use toolbox::vector;

use shared::pack_color;
use shared::should_exit;
use shared::Triangle;
use shared::Vertex;

const WIDTH: usize = 1520;
const HEIGHT: usize = 855;

fn main() {
    let mut buffer = Buffer2::new(WIDTH, HEIGHT, vector!(0.1, 0.1, 0.15));

    #[rustfmt::skip]
    let triangle_base = Triangle {
        vertices: [
            Vertex { pos: vector!(-0.5, -0.5, 0), col: vector!(1  , 0.7, 0  ) },
            Vertex { pos: vector!(0.5 , -0.5, 0), col: vector!(0  , 1  , 0.7) },
            Vertex { pos: vector!(0   , 0.5 , 0), col: vector!(0.7, 0  , 1  ) },
        ],
        pos: vector!(0, 0, 10),
        rot: vector!(0, 0, 0),
        scale: 9.,
    };
    let mut triangle = triangle_base;

    benchmark(&mut buffer, &mut triangle);

    let window_handle = Window::new(
        "historic rasterizers",
        WIDTH,
        HEIGHT,
        WindowOptions { resize: false, scale: Scale::X1, ..Default::default() },
    );
    let mut window = window_handle.expect("failed to open window");
    window.set_target_fps(9999999);

    interactive(buffer, triangle, window);
}

fn benchmark(buffer: &mut Buffer2<Vector3<f32>>, triangle: &mut Triangle) {
    let envs: Vec<String> = std::env::args().collect();
    let benchmark = envs.get(1).unwrap();
    let target = envs.get(2).unwrap().parse::<i32>().unwrap();
    #[allow(clippy::type_complexity)]
    let func: Option<fn(&mut Buffer2<Vector3<f32>>, &Triangle)> = match benchmark.as_str() {
        | "barycentric" => Some(barycentric::render),
        | "scanline" => Some(scanline::render),
        | "raytraced" => Some(raytraced::render),
        | _ => None,
    };

    if let Some(func) = func {
        let starting = std::time::Instant::now();
        (0..target).for_each(|_| {
            triangle.rot = triangle.rot + vector!(0.02, 0.01, 0.003);
            (func)(buffer, &*triangle);
        });
        let ending = starting.elapsed();
        println!(
            "{} took {} s to render {} triangles\naverage time per triangle: {} microseconds",
            benchmark,
            ending.as_secs_f64(),
            target,
            ending.as_micros() as f32 / target as f32,
        );
    }
}

fn interactive(mut buffer: Buffer2<Vector3<f32>>, mut triangle: Triangle, mut window: Window) {
    while !should_exit(&window) {
        triangle.rot = triangle.rot + vector!(0.02, 0.01, 0.003);

        buffer.clear();
        window.get_keys().iter().for_each(|key| match key {
            | Key::Q => barycentric::render(&mut buffer, &triangle),
            | Key::W => scanline::render(&mut buffer, &triangle),
            | Key::E => raytraced::render(&mut buffer, &triangle),
            | _ => {}
        });

        window
            .update_with_buffer(
                &buffer.data.iter().map(pack_color).collect::<Vec<u32>>(),
                buffer.width,
                buffer.height,
            )
            .expect("failed to update buffer");
    }
}
