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
    fn get_cell(&self, pos: Vec2<isize>) -> Option<(&Celula, f32)> {
        let idx = self.indice_linear(pos)?;
        let cel = self.celulas.get(idx)?;
        let z = *self.get_z(pos)?;
        Some((cel, z))
    }
    fn get_cell_mut(&mut self, pos: Vec2<isize>) -> Option<(&mut Celula, f32)> {
        let idx = self.indice_linear(pos)?;
        let z = *self.get_z(pos)?;
        let cel = self.celulas.get_mut(idx)?;
        Some((cel, z))
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

    pub fn set_pixel(&mut self, pos: Vec2<isize>, cor: Color, z: f32) {
        let celula = self.get_cell_mut(pos);

        if let Some((cel, z_b)) = celula {
            if z_b > z {
                return;
            }
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

                if let Some((cel, _)) = celula {
                    let fg = cel.fg;
                    let bg = cel.bg;
                    let ch = cel.ch;

                    queue!(term, SetColors(Colors::new(fg, bg)), Print(ch), ResetColor,)?;
                    //write!(term, "{cor}{ch}\x1b[0m", cor = Cor::codigo(fg, bg))?;
                }
            }
            if y < altura - 1 {
                queue!(term, MoveToNextLine(1))?;
                // write!(term, "\r\n")?;
            }
        }

        Ok(())
    }
}
