use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector2;
use toolbox::math::vector::Vector3;
use toolbox::vector;

use crate::shared::BarycentricSystem;
use crate::shared::Triangle;
use crate::shared::Vec3f;

pub struct BoundingBox<T> {
    mins: Vector2<T>,
    maxs: Vector2<T>,
}

impl Triangle {
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

pub fn render(buffer: &mut Buffer2<Vec3f>, triangle: &Triangle) {
    let triangle = triangle.screen_transform(buffer.width, buffer.height);

    let bounds = triangle.bounds();
    let system = BarycentricSystem::from(&triangle);

    let [a, b, c] = &triangle.vertices;
    let inv_depths = vector!(1. / a.pos.z, 1. / b.pos.z, 1. / c.pos.z);
    let r_pre = vector!(a.col.x * inv_depths.x, b.col.x * inv_depths.y, c.col.x * inv_depths.z);
    let g_pre = vector!(a.col.y * inv_depths.x, b.col.y * inv_depths.y, c.col.y * inv_depths.z);
    let b_pre = vector!(a.col.z * inv_depths.x, b.col.z * inv_depths.y, c.col.z * inv_depths.z);

    for y in bounds.mins.y..bounds.maxs.y {
        let lambdas_start = system.calculate_point(vector!(bounds.mins.x, y) + vector!(0.5, 0.5));
        let lambdas_end = system.calculate_point(vector!(bounds.maxs.x, y) + vector!(0.5, 0.5));
        let lambda_step = (lambdas_end - lambdas_start) / (bounds.maxs.x as f32 - bounds.mins.x as f32);

        let mut lambdas = lambdas_start;
        for x in bounds.mins.x..bounds.maxs.x {
            let weights = lambdas;
            lambdas = lambdas + lambda_step;

            if !system.within_triangle(weights) {
                continue;
            }

            let depth = (inv_depths * weights).recip();

            let r = r_pre * weights * depth;
            let g = g_pre * weights * depth;
            let b = b_pre * weights * depth;

            buffer.set(x, y, vector!(r, g, b));
        }
    }
}
