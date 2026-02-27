pub mod buffer;
pub mod camera;

use crate::vetores::*;
use buffer::Buffer;
use camera::Camera;

use crossterm::style::Color;

pub trait Renderizavel {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer);
}

pub struct Vertice {
    pub pos: Vec3<f32>,
}
impl Renderizavel for Vertice {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer) {
        let pos_proj = cam.projetar(self.pos);

        if let Some((pos, z)) = pos_proj {
            buf.set_pixel(pos, Color::Cyan, z);
        }
    }
}

#[derive(Debug)]
pub struct Aresta {
    pub de: Vec3<f32>,
    pub ate: Vec3<f32>,
}
impl Renderizavel for Aresta {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer) {
        let ((a, _az), (b, _bz)) = match cam.projetar_e_clipar_linha(self.de, self.ate) {
            Some(l) => (l[0], l[1]),
            None => return,
        };

        let mut x0 = a.x;
        let mut y0 = a.y;
        let x1 = b.x;
        let y1 = b.y;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            buf.set_pixel(Vec2::new(x0, y0), Color::White, 0.5);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x0 += sx;
            }

            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
