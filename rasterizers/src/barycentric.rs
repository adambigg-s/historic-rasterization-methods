use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector2;
use toolbox::math::vector::Vector3;
use toolbox::vec2;
use toolbox::vec3;
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

    for y in bounds.mins.y..bounds.maxs.y {
        for x in bounds.mins.x..bounds.maxs.x {
            let point = vector!(x, y) + vector!(0.5, 0.5);

            let lambdas = system.calculate_point(point);
            if !(lambdas.x > 0. && lambdas.y > 0. && lambdas.z > 0.) {
                continue;
            }

            let [a, b, c] = &triangle.vertices;

            let inv_depths = vector!(1. / a.pos.z, 1. / b.pos.z, 1. / c.pos.z);
            let depth = (inv_depths * lambdas).recip();

            let r = vector!(a.col.x, b.col.x, c.col.x) * lambdas;
            let g = vector!(a.col.y, b.col.y, c.col.y) * lambdas;
            let b = vector!(a.col.z, b.col.z, c.col.z) * lambdas;

            buffer.set(x, y, vector!(r, g, b));
        }
    }
}
