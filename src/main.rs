mod matematica;
mod renderizacao;

use std::io::stdin;

use matematica::{Vec2, Vec3};
use renderizacao::{Buffer, Camera, Renderizavel, Vertice};

use crate::renderizacao::Aresta;

fn main() -> std::io::Result<()> {
    let mut term = std::io::stdout();
    let stdin = std::io::stdin();
    let res = Vec2::new(80, 25);
    let cam = Camera::new(Vec3::new(0., 0., 0.), 1.0, res);
    let mut buf = Buffer::new(res);

    let mut vert = Vertice {
        pos: Vec3::new(0., 0., 1.),
    };
    let mut ares = Aresta {
        de: Vec3::new(10., 5., 5.),
        ate: Vec3::new(-10., -5., 5.),
    };

    let mut a = 1.0_f32;

    loop {
        buf.limpar();

        vert.renderizar(&cam, &mut buf);
        ares.renderizar(&cam, &mut buf);

        buf.renderizar(&mut term)?;

        vert.pos.x = a.sin() * 10.0;
        ares.de.y = 4. + a.sin() * 60.;
        a += 0.5;

        // stdin.read_line(&mut String::new())?;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(())
}
