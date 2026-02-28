mod render;
mod vetores;

use std::time::Duration;

use crossterm::{
    cursor,
    event::{self},
    execute,
    style::Color,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use render::{Aresta, Renderizavel, Vertice, buffer::Buffer, camera::Camera};
use vetores::{Vec2, Vec3};

use crate::render::Poligono;

fn main() -> std::io::Result<()> {
    let mut term = std::io::stdout();

    enable_raw_mode()?;
    execute!(term, terminal::EnterAlternateScreen, cursor::Hide)?;

    let tela = terminal::size()?;
    let res = Vec2::new(tela.0 as usize, tela.1 as usize * 2);
    let cam = Camera::new(Vec3::new(0., 0., 0.), 1.0, res);
    let mut buf = Buffer::new(res);

    let cubo = vec![
        vec![
            Vec3::new(40., 40., 5.),
            Vec3::new(-40., 40., 5.),
            Vec3::new(-40., 40., 10.),
            Vec3::new(40., 40., 10.),
        ],
        vec![
            Vec3::new(40., -40., 5.),
            Vec3::new(-40., -40., 5.),
            Vec3::new(-40., -40., 10.),
            Vec3::new(40., -40., 10.),
        ],
    ];

    let mut t = 0.0_f32;

    let pol = Poligono {
        vertices: vec![
            Vec3::new(50., 1., 1.),
            Vec3::new(25., 80., 700.),
            Vec3::new(10., 10., 1.),
        ],
    };

    loop {
        buf.limpar(None);

        pol.renderizar(&cam, &mut buf);

        t += 0.1;
        let offset = t.sin() * 10.0;

        if event::poll(Duration::from_millis(50))? {
            if event::read()?.is_key_press() {
                break;
            }
        }

        for v in 0..4 {
            for face in cubo.iter() {
                let (ax, ay, az) = face[v].into();
                let (bx, by, bz) = face[(v + 1) % 4].into();

                let a = Vec3::new(ax + offset, ay + offset, az + offset);
                let b = Vec3::new(bx + offset, by + offset, bz + offset);

                let aresta = Aresta { de: a, ate: b };

                aresta.renderizar(&cam, &mut buf);
            }
            let (ax, ay, az) = cubo[0][v].into();
            let (bx, by, bz) = cubo[1][v].into();

            let a = Vec3::new(ax + offset, ay + offset, az + offset);
            let b = Vec3::new(bx + offset, by + offset, bz + offset);

            let aresta = Aresta { de: a, ate: b };

            aresta.renderizar(&cam, &mut buf);
        }

        buf.renderizar(&mut term)?;
    }

    disable_raw_mode()?;
    execute!(term, terminal::LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
