use crate::matematica::{Vec2, Vec3};
use std::io::{Result, Stdout, Write};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Cor {
    Preto = 0,
    Vermelho = 1,
    Verde = 2,
    Amarelo = 3,
    Azul = 4,
    Magenta = 5,
    Ciano = 6,
    Branco = 7,
}

impl Cor {
    const ESCAPE: char = '\x1b';

    pub fn codigo(fg: Self, bg: Self) -> String {
        let esc = Cor::ESCAPE;
        let fg = fg as u8;
        let bg = bg as u8;
        format!("{esc}[3{fg};4{bg}m")
    }
}

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
            buf.set_pixel(pos, Cor::Ciano);
        }
    }
}

pub struct Aresta {
    pub de: Vec3<f32>,
    pub ate: Vec3<f32>,
}
impl Renderizavel for Aresta {
    fn renderizar(&self, cam: &Camera, buf: &mut Buffer) {
        let (de, ate) = match cam.projetar_e_clipar_linha(self.de, self.ate) {
            Some(l) => (l[0].0, l[1].0),
            None => return,
        };

        let mut x0 = de.x;
        let mut y0 = de.y;
        let x1 = ate.x;
        let y1 = ate.y;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            buf.set_pixel(Vec2::new(x0, y0), Cor::Verde);

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

#[derive(Clone, Copy, Debug)]
struct Celula {
    ch: char,
    fg: Cor,
    bg: Cor,
}

pub struct Buffer {
    resolucao: Vec2<usize>,
    celulas: Vec<Celula>,
}

const MEIO_BLOCO_CIMA: char = '▀';
const MEIO_BLOCO_BAIXO: char = '▄';

impl Buffer {
    pub fn new(resolucao: Vec2<usize>) -> Self {
        let celula = Celula {
            ch: MEIO_BLOCO_CIMA,
            fg: Cor::Preto,
            bg: Cor::Preto,
        };
        let celulas = vec![celula; resolucao.x * (resolucao.y / 2)];

        Buffer { resolucao, celulas }
    }

    pub fn limpar(&mut self) {
        for cel in self.celulas.iter_mut() {
            cel.fg = Cor::Preto;
            cel.bg = Cor::Preto;
        }
    }

    fn indice_linear(&self, pos: Vec2<isize>) -> Option<usize> {
        let (x, y) = pos.into();

        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        let y_terminal = y / 2;

        if x >= self.resolucao.x || y_terminal >= (self.resolucao.y / 2) {
            return None;
        }

        Some(y_terminal * self.resolucao.x + x)
    }

    fn get_cell(&self, pos: Vec2<isize>) -> Option<&Celula> {
        let idx = self.indice_linear(pos)?;
        self.celulas.get(idx)
    }
    fn get_cell_mut(&mut self, pos: Vec2<isize>) -> Option<&mut Celula> {
        let idx = self.indice_linear(pos)?;
        self.celulas.get_mut(idx)
    }

    fn set_celula(&mut self, pos: Vec2<usize>, celula: Celula) {
        let idx = pos.y * self.resolucao.x + pos.x;

        if idx >= (self.resolucao.x * self.resolucao.y) {
            return;
        }

        self.celulas[idx] = celula;
    }

    fn set_pixel(&mut self, pos: Vec2<isize>, cor: Cor) {
        let celula = self.get_cell_mut(pos);

        if let Some(cel) = celula {
            if pos.y % 2 == 0 {
                cel.fg = cor;
            } else {
                cel.bg = cor;
            }
        }
    }

    pub fn renderizar(&self, term: &mut Stdout) -> Result<()> {
        for y in 0..(self.resolucao.y / 2) {
            for x in 0..self.resolucao.x {
                let (x, y) = (x as isize, y as isize);
                let celula = self.get_cell(Vec2::new(x, y * 2));

                if let Some(cel) = celula {
                    let fg = cel.fg;
                    let bg = cel.bg;
                    let ch = cel.ch;

                    write!(term, "{cor}{ch}\x1b[0m", cor = Cor::codigo(fg, bg))?;
                }
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
}
impl Camera {
    pub fn new(pos: Vec3<f32>, dist_foco: f32, resolucao: Vec2<usize>) -> Self {
        Camera {
            pos,
            dist_foco,
            resolucao,
        }
    }
    pub fn projetar(&self, ponto: Vec3<f32>) -> Option<(Vec2<isize>, f32)> {
        let ponto_local = ponto - self.pos;

        self.projetar_local(ponto_local)
    }

    pub fn projetar_local(&self, ponto: Vec3<f32>) -> Option<(Vec2<isize>, f32)> {
        let (x, y, z) = ponto.into();

        if z < self.dist_foco {
            return None;
        }

        let aspecto_pixel = 1.0 / 1.0;
        let offset_h = self.resolucao.x as f32 / 2.0;
        let offset_v = self.resolucao.y as f32 / 2.0;

        let x_proj = self.dist_foco * (x / z) + offset_h;
        let y_proj = (self.dist_foco * (y / z) * aspecto_pixel) + offset_v;

        let x_proj = x_proj as isize;
        let y_proj = y_proj as isize;

        Some((Vec2::new(x_proj, y_proj), z))
    }

    pub fn clipar_linha(&self, p1: Vec3<f32>, p2: Vec3<f32>) -> Option<[Vec3<f32>; 2]> {
        let mut a = p1 - self.pos;
        let mut b = p2 - self.pos;

        let limite = self.dist_foco;

        if a.z < limite && b.z < limite {
            // atrás da camera
            return None;
        }

        if a.z < limite {
            let t = (limite - a.z) / (b.z - a.z);
            a = Vec3 {
                x: a.x + t * (b.x - a.x),
                y: a.y + t * (b.y - a.y),
                z: limite,
            };
        } else if b.z < limite {
            let t = (limite - a.z) / (b.z - a.z);
            b = Vec3 {
                x: a.x + t * (b.x - a.x),
                y: a.y + t * (b.y - a.y),
                z: limite,
            };
        }

        Some([a, b])
    }

    pub fn projetar_e_clipar_linha(
        &self,
        a: Vec3<f32>,
        b: Vec3<f32>,
    ) -> Option<[(Vec2<isize>, f32); 2]> {
        let [ca, cb] = self.clipar_linha(a, b)?;

        let (p1, z1) = self.projetar_local(ca)?;
        let (p2, z2) = self.projetar_local(cb)?;

        Some([(p1, z1), (p2, z2)])
    }
}
