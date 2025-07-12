use std::ops::Sub;

use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector2;
use toolbox::math::vector::Vector3;
use toolbox::vector;

use crate::shared::Interpolator;
use crate::shared::Triangle;
use crate::shared::Vec2f;
use crate::shared::Vec3f;

pub struct BoundingBox<T> {
    mins: Vector2<T>,
    maxs: Vector2<T>,
}

impl<T> BoundingBox<T>
where
    T: Sub<Output = T> + Clone + Copy,
{
    #[inline]
    pub fn width(&self) -> T {
        self.maxs.x - self.mins.x
    }

    #[inline]
    pub fn height(&self) -> T {
        self.maxs.y - self.mins.y
    }
}

impl Triangle {
    #[inline]
    pub fn bounds(&self) -> BoundingBox<usize> {
        let (xs, ys) = (
            self.vertices.map(|vertex| vertex.pos.x as usize),
            self.vertices.map(|vertex| vertex.pos.y as usize),
        );

        BoundingBox {
            mins: Vector2::build(*xs.iter().min().unwrap(), *ys.iter().min().unwrap()),
            maxs: Vector2::build(*xs.iter().max().unwrap(), *ys.iter().max().unwrap()),
        }
    }
}

pub struct BarycentricSystem {
    a: Vec2f,
    b: Vec2f,
    c: Vec2f,
    ba: Vec2f,
    cb: Vec2f,
    ac: Vec2f,
}

impl BarycentricSystem {
    #[inline]
    pub fn from(triangle: &Triangle) -> Self {
        let [a, b, c] = triangle.vertices.map(|vertex| vector!(vertex.pos.x.floor(), vertex.pos.y.floor()));

        Self { a, b, c, ba: b - a, cb: c - b, ac: a - c }
    }

    #[inline]
    pub fn calculate_point(&self, point: Vec2f) -> Vec3f {
        let (ap, bp, cp) = (point - self.a, point - self.b, point - self.c);

        let (apb, bpc, cpa) = (self.ba % ap, self.cb % bp, self.ac % cp);

        let weights = vector!(bpc, cpa, apb);
        let area = bpc + cpa + apb;

        weights / area
    }

    #[inline]
    pub fn within_triangle(&self, lambdas: Vec3f) -> bool {
        lambdas.x >= 0. && lambdas.y >= 0. && lambdas.z >= 0.
    }
}

pub fn render(buffer: &mut Buffer2<Vec3f>, triangle: &Triangle) {
    let triangle = triangle.screen_transform(buffer.width, buffer.height);

    let bounds = triangle.bounds();
    let system = BarycentricSystem::from(&triangle);
    let width = bounds.width() as f32;
    let height = bounds.height() as f32;

    let [a, b, c] = &triangle.vertices;
    let inv_depths = vector!(1. / a.pos.z, 1. / b.pos.z, 1. / c.pos.z);
    let r_pre = vector!(a.col.x * inv_depths.x, b.col.x * inv_depths.y, c.col.x * inv_depths.z);
    let g_pre = vector!(a.col.y * inv_depths.x, b.col.y * inv_depths.y, c.col.y * inv_depths.z);
    let b_pre = vector!(a.col.z * inv_depths.x, b.col.z * inv_depths.y, c.col.z * inv_depths.z);

    let lambda00 = system.calculate_point(vector!(bounds.mins.x, bounds.mins.y) + vector!(0.5, 0.5));
    let lambda10 = system.calculate_point(vector!(bounds.mins.x, bounds.maxs.y) + vector!(0.5, 0.5));
    let lambda01 = system.calculate_point(vector!(bounds.maxs.x, bounds.mins.y) + vector!(0.5, 0.5));
    let lambda11 = system.calculate_point(vector!(bounds.maxs.x, bounds.maxs.y) + vector!(0.5, 0.5));

    let mut lambda_left = Interpolator::build(lambda00, lambda10, height);
    let mut lambda_right = Interpolator::build(lambda01, lambda11, height);

    for y in bounds.mins.y..bounds.maxs.y {
        let mut lambdas = Interpolator::build(lambda_left.curr, lambda_right.curr, width);

        for x in bounds.mins.x..bounds.maxs.x {
            let weights = lambdas.curr;
            lambdas.progress();

            if !system.within_triangle(weights) {
                continue;
            }

            let depth = (inv_depths * weights).recip();

            let r = r_pre * weights * depth;
            let g = g_pre * weights * depth;
            let b = b_pre * weights * depth;

            buffer.set(x, y, vector!(r, g, b));
        }

        lambda_left.progress();
        lambda_right.progress();
    }
}
