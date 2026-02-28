use crate::vetores::*;
use crossterm::{
    cursor::{self, MoveToNextLine},
    execute, queue,
    style::{Color, Colors, Print, ResetColor, SetColors},
};
use std::io;

const MEIO_BLOCO_CIMA: char = '▀';
const COR_PADRAO: Color = Color::Black;

#[derive(Clone, Copy, Debug)]
struct Celula {
    ch: char,
    fg: Color,
    bg: Color,
}
impl Default for Celula {
    fn default() -> Self {
        Celula {
            ch: MEIO_BLOCO_CIMA,
            fg: COR_PADRAO,
            bg: COR_PADRAO,
        }
    }
}

pub struct Buffer {
    resolucao: Vec2<usize>,
    celulas: Vec<Celula>,
    z_buffer: Vec<f32>,
}
impl Buffer {
    pub fn new(resolucao: Vec2<usize>) -> Self {
        let celulas = vec![Celula::default(); resolucao.x * (resolucao.y / 2)];
        let z_buffer = vec![0.0; resolucao.x * resolucao.y];

        Buffer {
            resolucao,
            celulas,
            z_buffer,
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

    fn set_z(&mut self, pos: Vec2<isize>, z: f32) -> Option<()> {
        let idx = self.indice_linear(pos)?;
        self.z_buffer[idx] = z;
        None
    }
    fn get_z(&self, pos: Vec2<isize>) -> Option<&f32> {
        let idx = self.indice_linear(pos)?;
        self.z_buffer.get(idx)
    }
    fn get_cell(&self, pos: Vec2<isize>) -> Option<&Celula> {
        let idx = self.indice_linear(pos)?;
        let cel = self.celulas.get(idx)?;
        Some(cel)
    }
    fn get_cell_mut(&mut self, pos: Vec2<isize>) -> Option<&mut Celula> {
        let idx = self.indice_linear(pos)?;
        let cel = self.celulas.get_mut(idx)?;
        Some(cel)
    }

    fn set_celula(&mut self, pos: Vec2<usize>, celula: Celula) {
        let idx = pos.y * self.resolucao.x + pos.x;

        if idx >= (self.resolucao.x * self.resolucao.y) {
            return;
        }

        self.celulas[idx] = celula;
    }

    pub fn limpar(&mut self, cor: Option<Color>) {
        let cor = cor.unwrap_or(COR_PADRAO);
        for cel in self.celulas.iter_mut() {
            cel.bg = cor;
            cel.fg = cor;
        }
    }

    fn set_pixel(&mut self, pos: Vec2<isize>, cor: Color) {
        let celula = self.get_cell_mut(pos);

        if let Some(cel) = celula {
            if pos.y % 2 == 0 {
                cel.fg = cor;
            } else {
                cel.bg = cor;
            }
        }
    }

    pub fn renderizar(&self, term: &mut io::Stdout) -> io::Result<()> {
        execute!(term, cursor::MoveTo(0, 0))?;
        let altura = self.resolucao.y / 2;

        for y in 0..altura {
            for x in 0..self.resolucao.x {
                let (x, y) = (x as isize, y as isize);
                let celula = self.get_cell(Vec2::new(x, y * 2));

                if let Some(cel) = celula {
                    let fg = cel.fg;
                    let bg = cel.bg;
                    let ch = cel.ch;

                    queue!(term, SetColors(Colors::new(fg, bg)), Print(ch), ResetColor,)?;
                }
            }
            if y < altura - 1 {
                queue!(term, MoveToNextLine(1))?;
            }
        }

        Ok(())
    }
}

impl Buffer {
    pub fn desenhar_ponto(&mut self, p: Vec2<isize>, cor: Color) {
        self.set_pixel(p, cor);
    }

    pub fn desenhar_linha(&mut self, vertices: &[Vec2<isize>], cor: Color) {
        if vertices.len() < 2 {
            return;
        }
        for i in 1..(vertices.len()) {
            let a = vertices[i];
            let b = vertices[i - 1];
            self.desenhar_linha_simples(a, b, cor);
        }
    }

    fn desenhar_linha_simples(&mut self, a: Vec2<isize>, b: Vec2<isize>, cor: Color) {
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
            self.set_pixel(Vec2::new(x0, y0), cor);

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

    pub fn desenhar_poligono(&mut self, vertices: &[Vec2<isize>], cor: Color) {
        if vertices.len() < 3 {
            return;
        }

        for i in 1..(vertices.len() - 1) {
            self.rasterizar_triangulo_simples(vertices[0], vertices[i], vertices[i + 1], cor);
        }
    }

    fn rasterizar_triangulo_simples(
        &mut self,
        a: Vec2<isize>,
        b: Vec2<isize>,
        c: Vec2<isize>,
        cor: Color,
    ) {
        let min_x = a.x.min(b.x).min(c.x);
        let max_x = a.x.max(b.x).max(c.x);
        let min_y = a.y.min(b.y).min(c.y);
        let max_y = a.y.max(b.y).max(c.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x, y);
                if ponto_em_triangulo(p, a, b, c) {
                    self.set_pixel(p, cor);
                }
            }
        }
    }
}
fn ponto_em_triangulo(p: Vec2<isize>, a: Vec2<isize>, b: Vec2<isize>, c: Vec2<isize>) -> bool {
    fn side(p1: Vec2<isize>, p2: Vec2<isize>, p3: Vec2<isize>) -> isize {
        // edge function
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }
    let d1 = side(p, a, b);
    let d2 = side(p, b, c);
    let d3 = side(p, c, a);

    let tem_negativo = (d1 < 0) || (d2 < 0) || (d3 < 0);
    let tem_positivo = (d1 > 0) || (d2 > 0) || (d3 > 0);

    !(tem_negativo && tem_positivo)
}
