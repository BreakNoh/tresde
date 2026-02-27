mod render;
mod vetores;

use crossterm::{
    cursor, execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use render::{Aresta, Renderizavel, Vertice, buffer::Buffer, camera::Camera};
use vetores::{Vec2, Vec3};

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

    for _ in 0..100 {
        buf.limpar(None);

        t += 0.1;
        let offset = t.sin() * 10.0;

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
            // let a = cubo[0][v];
            // let b = cubo[1][v];

            let aresta = Aresta { de: a, ate: b };

            aresta.renderizar(&cam, &mut buf);
        }

        buf.renderizar(&mut term)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    disable_raw_mode()?;
    execute!(term, terminal::LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
