use crate::matematica::{Vec2, Vec3};
use std::io::{Result, Stdout, Write};

trait Renderizavel {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer);
}

struct Vertice {
    pos: Vec3<f32>,
}
impl Renderizavel for Vertice {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer) {
        let celula = Celula { ch: '0' };
        let pos_proj = cam.projetar(self.pos);

        if let Some((pos, z)) = pos_proj {
            buf.set_celula(pos, celula);
        }
    }
}

struct Aresta {
    de: Vec3<f32>,
    ate: Vec3<f32>,
}
impl Renderizavel for Aresta {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer) {
        if self.de.z <= cam.limite && self.ate.z <= cam.limite {
            return;
        }
        let de_proj = cam.projetar(self.de);
        let ate_proj = cam.projetar(self.ate);
    }
}

struct Renderizador {}

#[derive(Clone, Copy, Debug)]
struct Celula {
    ch: char,
}

struct Buffer {
    resolucao: Vec2<usize>,
    celulas: Vec<Celula>,
}
impl Buffer {
    fn new(resolucao: Vec2<usize>) -> Self {
        let celula = Celula { ch: '_' };
        let celulas = vec![celula; resolucao.x * resolucao.y];

        Buffer { resolucao, celulas }
    }

    fn set_celula(&mut self, pos: Vec2<usize>, celula: Celula) {
        let idx = pos.y * self.resolucao.x + pos.x;

        if idx >= (self.resolucao.x * self.resolucao.y) {
            return;
        }

        self.celulas[idx] = celula;
    }

    fn renderizar(&self, term: &mut Stdout) -> Result<()> {
        for y in 0..self.resolucao.y {
            for x in 0..self.resolucao.x {
                let idx = y * self.resolucao.x + x;
                let celula = self.celulas[idx].ch;

                write!(term, "{celula}")?;
            }
            write!(term, "\n")?;
        }

        Ok(())
    }
}

pub struct Camera {
    pos: Vec3<f32>,
    dist_foco: f32,
    resolucao: Vec2<usize>,
    limite: f32,
}
impl Camera {
    fn new(pos: Vec3<f32>, dist_foco: f32, resolucao: Vec2<usize>) -> Self {
        Camera {
            pos,
            dist_foco,
            resolucao,
            limite: 0.0,
        }
    }
    fn projetar(&self, ponto: Vec3<f32>) -> Option<(Vec2<usize>, f32)> {
        let (dx, dy, dz) = (ponto - self.pos).into();

        if dz <= self.limite {
            return None;
        }

        let aspecto_pixel = 1.0 / 2.0;
        let offset_h = self.resolucao.x as f32 / 2.0;
        let offset_v = self.resolucao.y as f32 / 2.0;

        let x_proj = self.dist_foco * (dx / dz) + offset_h;
        let y_proj = (self.dist_foco * (dy / dz) * aspecto_pixel) + offset_v;

        if x_proj < 0.0 || x_proj >= self.resolucao.x as f32 {
            return None;
        }
        if y_proj < 0.0 || y_proj >= self.resolucao.y as f32 {
            return None;
        }

        let x_proj = x_proj as usize;
        let y_proj = y_proj as usize;

        Some((Vec2::new(x_proj, y_proj), dz))
    }
}
