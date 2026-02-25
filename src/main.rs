mod matematica;
mod renderizacao;

use matematica::{Vec2, Vec3};

fn main() -> Result<()> {
    let mut term = std::io::stdout();
    let res = Vec2::new(80, 25);
    let cam = Camera::new(Vec3::new(0., 0., 0.), 1.0, res);
    let mut buf = Buffer::new(res);
    let vert = Vertice {
        pos: Vec3::new(0., 0., 1.),
    };

    vert.renderizar(&cam, &mut buf);

    buf.renderizar(&mut term)?;

    Ok(())
}
