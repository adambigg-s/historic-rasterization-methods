use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::FloatVectorOps;
use toolbox::math::vector::Vector3;
use toolbox::vector;

use crate::shared::Mat3f;
use crate::shared::Rotation3;
use crate::shared::Triangle;
use crate::shared::Vec3f;
use crate::shared::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3f,
    direc: Vec3f,
}

impl Ray {
    pub fn intersection(&self, triangle: &Triangle) -> Option<Vec3f> {
        const EPSILON: f32 = 1e-6;

        let [a, b, c] = triangle.vertices.map(|vertex| vertex.pos);
        let e1 = b - a;
        let e2 = c - a;
        let pvec = self.direc % e2;
        let det = e1 * pvec;
        if det.abs() < EPSILON {
            return None;
        }

        let tvec = self.origin - a;
        let qvec = tvec % e1;
        let inv_det = 1. / det;

        let u = (tvec * pvec) * inv_det;
        if !(0. ..=1.).contains(&u) {
            return None;
        }

        let v = (self.direc * qvec) * inv_det;
        if !(0. ..=1.).contains(&v) {
            return None;
        }

        if u + v > 1. {
            return None;
        }

        Some(vector!(1. - u - v, u, v))
    }
}

impl Triangle {
    pub fn world_transform(&self) -> Self {
        let xyz_transform = <Mat3f as Rotation3>::rotation_x(self.rot.x)
            * <Mat3f as Rotation3>::rotation_y(self.rot.y)
            * <Mat3f as Rotation3>::rotation_z(self.rot.z);

        let vertices = self.vertices.map(|vertex| {
            let mut position = vertex.pos;

            // scale
            position = position * self.scale;

            // attitude
            position = xyz_transform * position;

            // translation
            position = position + self.pos;

            Vertex { pos: position, col: vertex.col }
        });

        Self {
            vertices,
            pos: self.pos,
            rot: self.rot,
            scale: self.scale,
        }
    }
}

pub fn render(buffer: &mut Buffer2<Vec3f>, triangle: &Triangle) {
    let triangle = triangle.world_transform();

    let ar = buffer.width as f32 / buffer.height as f32;
    let [a, b, c] = triangle.vertices;
    let r_pre = vector!(a.col.x, b.col.x, c.col.x);
    let g_pre = vector!(a.col.y, b.col.y, c.col.y);
    let b_pre = vector!(a.col.z, b.col.z, c.col.z);

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let ndc_x = (x as f32 + 0.5) / buffer.width as f32 * 2. - 1.;
            let ndc_y = -(y as f32 + 0.5) / buffer.height as f32 * 2. + 1.;

            let ray = Ray {
                origin: vector!(0, 0, 0),
                direc: vector!(ndc_x * ar, ndc_y, 1).normalize(),
            };

            let Some(weights) = ray.intersection(&triangle)
            else {
                continue;
            };

            let r = r_pre * weights;
            let g = g_pre * weights;
            let b = b_pre * weights;

            buffer.set(x, y, vector!(r, g, b));
        }
    }
}
