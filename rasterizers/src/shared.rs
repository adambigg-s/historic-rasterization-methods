use minifb::Key;
use minifb::Window;

use toolbox::math::matrix::Matrix3;
use toolbox::math::vector::Vector2;
use toolbox::math::vector::Vector3;
use toolbox::vec2;
use toolbox::vec3;
use toolbox::vector;

pub type Vec2f = Vector2<f32>;

pub type Vec3f = Vector3<f32>;

pub type Mat3f = Matrix3<f32>;

pub trait Rotation3 {
    fn rotation_x(angle: f32) -> Self;

    fn rotation_y(angle: f32) -> Self;

    fn rotation_z(angle: f32) -> Self;
}

impl Rotation3 for Matrix3<f32> {
    fn rotation_x(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();

        #[rustfmt::skip]
        let out = Matrix3::build([
            [1., 0., 0.],
            [0., c , -s],
            [0., s , c ],
        ]);
        out
    }

    fn rotation_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();

        #[rustfmt::skip]
        let out = Matrix3::build([
            [c , 0., s ],
            [0., 1., 0.],
            [-s, 0., c ],
        ]);
        out
    }

    fn rotation_z(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();

        #[rustfmt::skip]
        let out = Matrix3::build([
            [c , -s, 0.],
            [s , c , 0.],
            [0., 0., 1.],
        ]);
        out
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3f,
    pub col: Vec3f,
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub pos: Vec3f,
    pub rot: Vec3f,
    pub scale: f32,
}

impl Triangle {
    pub fn screen_transform(&self, width: usize, height: usize) -> Self {
        let rotation = <Mat3f as Rotation3>::rotation_x(self.rot.x)
            * <Mat3f as Rotation3>::rotation_y(self.rot.y)
            * <Mat3f as Rotation3>::rotation_z(self.rot.z);

        let (hw, hh) = (width as f32 / 2., height as f32 / 2.);

        let vertices = self.vertices.map(|vertex| {
            let mut position = vertex.pos * self.scale;
            position = rotation * position;
            position = self.pos + position;

            position.x /= position.z;
            position.y /= position.z;

            let nx = position.x * hw + hw;
            let ny = -position.y * hh + hh;

            Vertex { pos: vector!(nx, ny, position.z), col: vertex.col }
        });

        Self {
            vertices,
            pos: self.pos,
            rot: self.rot,
            scale: self.scale,
        }
    }
}

pub struct BarycentricSystem {
    a: Vec2f,
    b: Vec2f,
    c: Vec2f,
}

impl BarycentricSystem {
    pub fn from(triangle: &Triangle) -> Self {
        let [a, b, c] = triangle.vertices.map(|vertex| vector!(vertex.pos.x.floor(), vertex.pos.y.floor()));

        Self { a, b, c }
    }

    pub fn calculate_point(&self, point: Vec2f) -> Vec3f {
        let (ap, bp, cp) = (point - self.a, point - self.b, point - self.c);

        let (apb, bpc, cpa) = ((self.b - self.a) % ap, (self.c - self.b) % bp, (self.a - self.c) % cp);

        let weights = vector!(bpc, cpa, apb);
        let area = apb + bpc + cpa;

        weights / area
    }

    pub fn within_triangle(&self, lambdas: Vec3f) -> bool {
        lambdas.x >= 0. && lambdas.y >= 0. && lambdas.z >= 0.
    }
}

pub fn pack_color(color: &Vec3f) -> u32 {
    let r = (color.x * 255.) as u32;
    let g = (color.y * 255.) as u32;
    let b = (color.z * 255.) as u32;

    (0xff << 24) | (r << 16) | (g << 8) | b
}

pub fn should_exit(window: &Window) -> bool {
    !window.is_open() || window.is_key_down(Key::Escape)
}
