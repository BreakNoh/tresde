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

        if let Some((pos, _)) = pos_proj {
            buf.desenhar_ponto(pos, Color::Cyan);
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
        if let Some([(a, _), (b, _)]) = cam.projetar_e_clipar_linha(self.de, self.ate) {
            buf.desenhar_linha(&vec![a, b], Color::White);
        };
    }
}
