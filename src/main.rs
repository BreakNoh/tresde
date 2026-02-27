mod matematica;
mod renderizacao;

use std::io::stdin;

use crossterm::{
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use matematica::{Vec2, Vec3};
use renderizacao::{Buffer, Camera, Renderizavel, Vertice};

use crate::renderizacao::Aresta;

fn main() -> std::io::Result<()> {
    let mut term = std::io::stdout();

    enable_raw_mode()?;
    execute!(term, terminal::EnterAlternateScreen)?;

    let stdin = std::io::stdin();
    let tela = terminal::size()?;
    let res = Vec2::new(tela.0 as usize, tela.1 as usize * 2);
    let cam = Camera::new(Vec3::new(0., 0., 0.), 1.0, res);
    let mut buf = Buffer::new(res);

    let t = vec![
        Vec3::new(-10., 50., 5.),
        Vec3::new(60., -120., 6.),
        Vec3::new(10., 20., 1.1),
        Vec3::new(-10., 50., 5.),
    ];

    let mut tri: Vec<Aresta> = vec![];

    let mut u = t[0];

    for i in 1..=3 {
        tri.push(Aresta { de: u, ate: t[i] });
        u = t[i];
    }

    let mut vert = Vertice {
        pos: Vec3::new(0., 0., 1.),
    };
    let mut ares = Aresta {
        de: Vec3::new(10., 5., 5.),
        ate: Vec3::new(-10., -5., 5.),
    };

    let mut a = 1.0_f32;

    for _ in 0..100 {
        buf.limpar();

        vert.renderizar(&cam, &mut buf);
        // ares.renderizar(&cam, &mut buf);

        for are in tri.iter() {
            are.renderizar(&cam, &mut buf);
        }

        vert.pos.x = a.sin() * 10.0;
        ares.de.y = 4. + a.sin() * 60.;
        a += 0.5;

        buf.renderizar(&mut term)?;
        // stdin.read_line(&mut String::new())?;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    disable_raw_mode()?;
    execute!(term, terminal::LeaveAlternateScreen)?;

    Ok(())
}
