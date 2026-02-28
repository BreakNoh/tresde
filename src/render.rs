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

#[derive(Debug)]
pub struct Poligono {
    pub vertices: Vec<Vec3<f32>>,
}

impl Renderizavel for Poligono {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer) {
        // 1. O clipping é vital para evitar distorções matemáticas
        let vertices_locais = cam.clipar_poligono(&self.vertices);

        if vertices_locais.len() < 3 {
            return;
        }

        // 2. Projetar vértices para o ecrã
        let mut pontos_2d = Vec::new();
        // let mut z_acumulado = 0.0;

        for v in &vertices_locais {
            if let Some((pos, _)) = cam.projetar_local(*v) {
                pontos_2d.push(pos);
                // z_acumulado += z;
            }
        }

        if pontos_2d.len() < 3 {
            return;
        }

        // Z médio para o Z-buffer (simplificação inicial)
        // let z_final = z_acumulado / pontos_2d.len() as f32;

        // 3. Desenhar no buffer
        buf.desenhar_poligono(&pontos_2d, Color::Green);
    }
}
