use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector2;
use toolbox::math::vector::Vector3;
use toolbox::vector;

use crate::shared::Interpolator;
use crate::shared::Triangle;
use crate::shared::Vec3f;
use crate::shared::Vertex;

impl Triangle {
    #[inline(always)]
    pub fn vertex_vertical_sort(&mut self) {
        let [a, b, c] = &mut self.vertices;
        if a.pos.y > b.pos.y {
            std::mem::swap(a, b);
        }
        if b.pos.y > c.pos.y {
            std::mem::swap(b, c);
        }
        if a.pos.y > b.pos.y {
            std::mem::swap(a, b);
        }
    }
}

pub type ScreenVertex = Vertex;

impl ScreenVertex {
    #[inline(always)]
    pub fn build(x: usize, y: usize, inv_depth: f32, inv_color: Vec3f) -> Self {
        Self { pos: vector!(x, y, inv_depth), col: inv_color }
    }
}

impl Add for ScreenVertex {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self { pos: self.pos + other.pos, col: self.col + other.col }
    }
}

impl Sub for ScreenVertex {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self { pos: self.pos - other.pos, col: self.col - other.col }
    }
}

impl Div<f32> for ScreenVertex {
    type Output = Self;

    #[inline(always)]
    fn div(self, scalar: f32) -> Self::Output {
        Self { pos: self.pos / scalar, col: self.col / scalar }
    }
}

pub fn render(buffer: &mut Buffer2<Vec3f>, triangle: &Triangle) {
    let mut triangle = triangle.screen_transform(buffer.width, buffer.height);
    triangle.vertex_vertical_sort();

    let [v0, v1, v2] = triangle.vertices;
    let [p0, p1, p2] = triangle.vertices.map(|vertex| vector!(usize; vertex.pos.x, vertex.pos.y));

    let invd_a = 1. / v0.pos.z;
    let invd_b = 1. / v1.pos.z;
    let invd_c = 1. / v2.pos.z;

    let (screena, screenb, screenc) = (
        ScreenVertex::build(p0.x, p0.y, invd_a, v0.col * invd_a),
        ScreenVertex::build(p1.x, p1.y, invd_b, v1.col * invd_b),
        ScreenVertex::build(p2.x, p2.y, invd_c, v2.col * invd_c),
    );

    let mut short_side = Interpolator::build(screena, screenb, screenb.pos.y - screena.pos.y);
    let mut long_side = Interpolator::build(screena, screenc, screenc.pos.y - screena.pos.y);
    let mut y = p0.y;
    while y <= p1.y {
        scanline(buffer, &short_side, &long_side, y);

        short_side.progress();
        long_side.progress();
        y += 1;
    }

    let mut short_side = Interpolator::build(screenc, screenb, screenc.pos.y - screenb.pos.y);
    let mut long_side = Interpolator::build(screenc, screena, screenc.pos.y - screena.pos.y);
    let mut y = p2.y;
    while y > p1.y {
        scanline(buffer, &short_side, &long_side, y);

        short_side.progress();
        long_side.progress();
        y -= 1;
    }
}

#[inline(always)]
fn scanline(buffer: &mut Buffer2<Vec3f>, p0: &Interpolator<Vertex>, p1: &Interpolator<Vertex>, y: usize) {
    let (start, end) = match p0.curr.pos.x < p1.curr.pos.x {
        | true => (p0.curr, p1.curr),
        | false => (p1.curr, p0.curr),
    };

    let mut inv_depth = Interpolator::build(start.pos.z, end.pos.z, end.pos.x - start.pos.x);
    let mut inv_col = Interpolator::build(start.col, end.col, end.pos.x - start.pos.x);
    for x in start.pos.x as usize..end.pos.x as usize {
        let depth = inv_depth.curr.recip();
        let color = inv_col.curr * depth;

        buffer.set(x, y, color);

        inv_depth.progress();
        inv_col.progress();
    }
}
