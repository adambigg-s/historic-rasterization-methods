use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

use minifb::Key;
use minifb::Window;

use toolbox::math::matrix::Matrix3;
use toolbox::math::vector::Vector2;
use toolbox::math::vector::Vector3;
use toolbox::matrix;
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
        let out = matrix!(
            1, 0, 0 ,
            0, c, -s,
            0, s, c
        );
        out
    }

    fn rotation_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();

        #[rustfmt::skip]
        let out = matrix!(
            c , 0, s,
            0 , 1, 0,
            -s, 0, c
        );
        out
    }

    fn rotation_z(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();

        #[rustfmt::skip]
        let out = matrix!(
            c, -s, 0,
            s, c , 0,
            0, 0 , 1
        );
        out
    }
}

pub struct Interpolator<T> {
    pub step: T,
    pub curr: T,
}

#[allow(dead_code)]
impl<T> Interpolator<T>
where
    T: Add<Output = T> + Sub<Output = T> + Clone + Copy,
{
    #[inline(always)]
    pub fn build<D>(start: T, end: T, steps: D) -> Self
    where
        T: Div<D, Output = T>,
    {
        Self { step: (end - start) / steps, curr: start }
    }

    #[inline(always)]
    pub fn progress(&mut self) -> T {
        self.curr = self.curr + self.step;
        self.curr
    }

    #[inline(always)]
    pub fn regress(&mut self) -> T {
        self.curr = self.curr - self.step;
        self.curr
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3f,
    pub col: Vec3f,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub pos: Vec3f,
    pub rot: Vec3f,
    pub scale: f32,
}

impl Triangle {
    pub fn screen_transform(&self, width: usize, height: usize) -> Self {
        let xyz_transform = <Mat3f as Rotation3>::rotation_x(self.rot.x)
            * <Mat3f as Rotation3>::rotation_y(self.rot.y)
            * <Mat3f as Rotation3>::rotation_z(self.rot.z);

        let (half_width, half_height) = (width as f32 / 2., height as f32 / 2.);
        let aspect_ratio = width as f32 / height as f32;

        let vertices = self.vertices.map(|vertex| {
            let mut position = vertex.pos;

            // scale
            position = position * self.scale;

            // attitude
            position = xyz_transform * position;

            // translation
            position = position + self.pos;

            // projection
            position.x /= position.z * aspect_ratio;
            position.y /= position.z;

            // screenspace conversion
            let nx = position.x * half_width + half_width;
            let ny = -position.y * half_height + half_height;

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

pub fn pack_color(color: &Vec3f) -> u32 {
    let r = (color.x * 255.) as u32;
    let g = (color.y * 255.) as u32;
    let b = (color.z * 255.) as u32;

    (0xff << 24) | (r << 16) | (g << 8) | b
}

pub fn should_exit(window: &Window) -> bool {
    !window.is_open() || window.is_key_down(Key::Escape)
}
