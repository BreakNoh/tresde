type Vec2<T> = (T, T);
type Vec3<T> = (T, T, T);

struct Camera {
    pos: Vec3<f32>,
    dist_foco: f32,
    resolucao: Vec2<usize>,
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
        let celula = Celula { ch: ' ' };
        let celulas = vec![celula; resolucao.0 * resolucao.1];

        Buffer { resolucao, celulas }
    }

    fn plotar(&mut self, pos: Vec2<usize>, celula: Celula) {
        let idx = pos.1 * self.resolucao.0 + pos.0;

        if idx >= (self.resolucao.0 * self.resolucao.1) {
            return;
        }

        self.celulas[idx] = celula;
    }
}

impl Camera {
    fn projetar(&self, ponto: Vec3<f32>) -> Option<(Vec2<usize>, f32)> {
        let (cx, cy, cz) = self.pos;
        let (px, py, pz) = ponto;

        let dx = px - cx;
        let dy = py - cy;
        let dz = pz - cz;

        if dz <= 0.1 {
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

fn main() {
    println!("Hello, world!");
}
