use toolbox::containers::buffer::Buffer2;
use toolbox::math::vector::Vector2;
use toolbox::vector;

use crate::shared::Triangle;
use crate::shared::Vec3f;

impl Triangle {
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

pub fn render(buffer: &mut Buffer2<Vec3f>, triangle: &Triangle) {
    let mut triangle = triangle.screen_transform(buffer.width, buffer.height);
    triangle.vertex_vertical_sort();

    let [v0, v1, v2] = triangle.vertices;
    let [a, b, c] = triangle.vertices.map(|vertex| vector!(usize; vertex.pos.x, vertex.pos.y));

    let inva = (b.x as f32 - a.x as f32) / (b.y as f32 - a.y as f32);
    let invb = (c.x as f32 - a.x as f32) / (c.y as f32 - a.y as f32);
    let mut cxa = a.x as f32;
    let mut cxb = a.x as f32;

    let mut cola = v0.col;
    let mut colb = v0.col;
    let colstepa = (v1.col - cola) / (b.y as f32 - a.y as f32);
    let colstepb = (v2.col - colb) / (c.y as f32 - a.y as f32);

    let mut y = a.y;
    while y <= b.y {
        let (xstart, xend) = match cxa > cxb {
            | true => (cxb as usize, cxa as usize),
            | false => (cxa as usize, cxb as usize),
        };
        let (colstart, colend) = match cxa > cxb {
            | true => (colb, cola),
            | false => (cola, colb),
        };

        let colstep = (colend - colstart) / (xend as f32 - xstart as f32);
        let mut color = colstart;
        for x in xstart..xend {
            buffer.set(x, y, color);
            color = color + colstep;
        }

        cola = cola + colstepa;
        colb = colb + colstepb;

        cxa += inva;
        cxb += invb;
        y += 1;
    }

    let inva = (b.x as f32 - c.x as f32) / (b.y as f32 - c.y as f32);
    let invb = (a.x as f32 - c.x as f32) / (a.y as f32 - c.y as f32);
    let mut cxa = c.x as f32;
    let mut cxb = c.x as f32;

    let mut cola = v2.col;
    let mut colb = v2.col;
    let colstepa = (v1.col - cola) / (b.y as f32 - c.y as f32);
    let colstepb = (v0.col - colb) / (a.y as f32 - c.y as f32);

    let mut y = c.y;
    while y > b.y {
        let (xstart, xend) = match cxa > cxb {
            | true => (cxb as usize, cxa as usize),
            | false => (cxa as usize, cxb as usize),
        };
        let (colstart, colend) = match cxa > cxb {
            | true => (colb, cola),
            | false => (cola, colb),
        };
        let colstep = (colend - colstart) / (xend as f32 - xstart as f32);
        let mut color = colstart;
        for x in xstart..xend {
            buffer.set(x, y, color);
            color = color + colstep;
        }

        cola = cola - colstepa;
        colb = colb - colstepb;

        cxa -= inva;
        cxb -= invb;
        y -= 1;
    }
}
