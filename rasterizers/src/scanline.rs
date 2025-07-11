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

    let invd_a = 1. / v0.pos.z;
    let invd_b = 1. / v1.pos.z;
    let invd_c = 1. / v2.pos.z;
    let inv_col0 = v0.col * invd_a;
    let inv_col1 = v1.col * invd_b;
    let inv_col2 = v2.col * invd_c;

    let mut cxa = a.x as f32;
    let mut cxb = a.x as f32;
    let inva = (b.x as f32 - a.x as f32) / (b.y as f32 - a.y as f32);
    let invb = (c.x as f32 - a.x as f32) / (c.y as f32 - a.y as f32);

    let mut cola = inv_col0;
    let mut colb = inv_col0;
    let colstepa = (inv_col1 - cola) / (b.y as f32 - a.y as f32);
    let colstepb = (inv_col2 - colb) / (c.y as f32 - a.y as f32);

    let mut deptha = invd_a;
    let mut depthb = invd_a;
    let depthstepa = (invd_b - invd_a) / (b.y as f32 - a.y as f32);
    let depthstepb = (invd_c - invd_a) / (c.y as f32 - a.y as f32);

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
        let (depth_start, depth_end) = match cxa > cxb {
            | true => (depthb, deptha),
            | false => (deptha, depthb),
        };

        let inv_colstep = (colend - colstart) / (xend as f32 - xstart as f32);
        let mut inv_color = colstart;

        let inv_depthstep = (depth_end - depth_start) / (xend as f32 - xstart as f32);
        let mut inv_depth = depth_start;

        for x in xstart..xend {
            let depth = 1. / inv_depth;
            let color = inv_color * depth;

            buffer.set(x, y, color);

            inv_color = inv_color + inv_colstep;
            inv_depth += inv_depthstep;
        }

        deptha += depthstepa;
        depthb += depthstepb;

        cola = cola + colstepa;
        colb = colb + colstepb;

        cxa += inva;
        cxb += invb;
        y += 1;
    }

    let mut cxa = c.x as f32;
    let mut cxb = c.x as f32;
    let inva = (b.x as f32 - c.x as f32) / (b.y as f32 - c.y as f32);
    let invb = (a.x as f32 - c.x as f32) / (a.y as f32 - c.y as f32);

    let mut cola = inv_col2;
    let mut colb = inv_col2;
    let colstepa = (inv_col1 - cola) / (b.y as f32 - c.y as f32);
    let colstepb = (inv_col0 - colb) / (a.y as f32 - c.y as f32);

    let mut deptha = invd_c;
    let mut depthb = invd_c;
    let depthstepa = (invd_b - invd_c) / (b.y as f32 - c.y as f32);
    let depthstepb = (invd_a - invd_c) / (a.y as f32 - c.y as f32);

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
        let (depth_start, depth_end) = match cxa > cxb {
            | true => (depthb, deptha),
            | false => (deptha, depthb),
        };

        let inv_colstep = (colend - colstart) / (xend as f32 - xstart as f32);
        let mut inv_color = colstart;

        let inv_depthstep = (depth_end - depth_start) / (xend as f32 - xstart as f32);
        let mut inv_depth = depth_start;

        for x in xstart..xend {
            let depth = 1. / inv_depth;
            let color = inv_color * depth;

            buffer.set(x, y, color);

            inv_color = inv_color + inv_colstep;
            inv_depth += inv_depthstep;
        }

        deptha -= depthstepa;
        depthb -= depthstepb;

        cola = cola - colstepa;
        colb = colb - colstepb;

        cxa -= inva;
        cxb -= invb;
        y -= 1;
    }
}
