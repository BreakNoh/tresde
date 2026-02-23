use std::io::{Result, Stdout, Write};

type Vec2<T> = (T, T);
type Vec3<T> = (T, T, T);

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
        if self.de.2 <= cam.limite && self.ate.2 <= cam.limite {
            return;
        }
        let de_proj = cam.projetar(self.de);
        let ate_proj = cam.projetar(self.ate);
    }
}

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
        let celulas = vec![celula; resolucao.0 * resolucao.1];

        Buffer { resolucao, celulas }
    }

    fn set_celula(&mut self, pos: Vec2<usize>, celula: Celula) {
        let idx = pos.1 * self.resolucao.0 + pos.0;

        if idx >= (self.resolucao.0 * self.resolucao.1) {
            return;
        }

        self.celulas[idx] = celula;
    }

    fn renderizar(&self, term: &mut Stdout) -> Result<()> {
        for y in 0..self.resolucao.1 {
            for x in 0..self.resolucao.0 {
                let idx = y * self.resolucao.0 + x;
                let celula = self.celulas[idx].ch;

                write!(term, "{celula}")?;
            }
            write!(term, "\n")?;
        }

        Ok(())
    }
}

struct Camera {
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
            limite: 0.1,
        }
    }
    fn projetar(&self, ponto: Vec3<f32>) -> Option<(Vec2<usize>, f32)> {
        let (cx, cy, cz) = self.pos;
        let (px, py, pz) = ponto;

        let dx = px - cx;
        let dy = py - cy;
        let dz = pz - cz;

        if dz <= self.limite {
            return None;
        }

        let aspecto_pixel = 1.0 / 2.0;
        let offset_h = self.resolucao.0 as f32 / 2.0;
        let offset_v = self.resolucao.1 as f32 / 2.0;

        let x_proj = self.dist_foco * (dx / dz) + offset_h;
        let y_proj = (self.dist_foco * (dy / dz) * aspecto_pixel) + offset_v;

        if x_proj < 0.0 || x_proj >= self.resolucao.0 as f32 {
            return None;
        }
        if y_proj < 0.0 || y_proj >= self.resolucao.1 as f32 {
            return None;
        }

        let x_proj = x_proj as usize;
        let y_proj = y_proj as usize;

        Some(((x_proj, y_proj), dz))
    }
}

fn main() -> Result<()> {
    let mut term = std::io::stdout();
    let res = (80, 25);
    let cam = Camera::new((0., 0., 0.), 1.0, res);
    let mut buf = Buffer::new(res);
    let vert = Vertice { pos: (0., 0., 1.) };

    // vert.renderizar(&cam, &mut buf);

    buf.renderizar(&mut term)?;

    Ok(())
}
