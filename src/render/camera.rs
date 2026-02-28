use crate::vetores::*;

fn interpolar_plano_z(a: Vec3<f32>, b: Vec3<f32>, z_plano: f32) -> Vec3<f32> {
    let t = (z_plano - a.z) / (b.z - a.z);
    Vec3 {
        x: a.x + t * (b.x - a.x),
        y: a.y + t * (b.y - a.y),
        z: z_plano,
    }
}

pub struct Camera {
    pos: Vec3<f32>,
    dist_foco: f32,
    resolucao: Vec2<usize>,
    alcance: f32,
}
impl Camera {
    pub fn new(pos: Vec3<f32>, dist_foco: f32, resolucao: Vec2<usize>) -> Self {
        Camera {
            pos,
            dist_foco,
            resolucao,
            alcance: 1000.0,
        }
    }

    pub fn projetar(&self, ponto: Vec3<f32>) -> Option<(Vec2<isize>, f32)> {
        let ponto_local = ponto - self.pos;

        self.projetar_local(ponto_local)
    }

    pub fn projetar_local(&self, ponto: Vec3<f32>) -> Option<(Vec2<isize>, f32)> {
        let (x, y, z) = ponto.into();

        if z < self.dist_foco || z > self.alcance {
            return None;
        }

        let aspecto_pixel = 1.0 / 1.0;
        let offset_h = self.resolucao.x as f32 / 2.0;
        let offset_v = self.resolucao.y as f32 / 2.0;

        let x_proj = self.dist_foco * (x / z) + offset_h;
        let y_proj = (self.dist_foco * (y / z) * aspecto_pixel) + offset_v;

        let x_proj = x_proj as isize;
        let y_proj = y_proj as isize;
        let z_norm = 1.0 - (z / self.alcance);

        Some((Vec2::new(x_proj, y_proj), z_norm))
    }

    pub fn clipar_linha(&self, p1: Vec3<f32>, p2: Vec3<f32>) -> Option<[Vec3<f32>; 2]> {
        let mut a = p1 - self.pos;
        let mut b = p2 - self.pos;

        let near = self.dist_foco;
        let far = self.alcance;

        if (a.z < near && b.z < near) || (a.z > far && b.z > far) {
            return None;
        }

        if a.z < near {
            a = interpolar_plano_z(a, b, near);
        } else if b.z < near {
            b = interpolar_plano_z(b, a, near);
        }

        if a.z > far {
            a = interpolar_plano_z(a, b, far);
        } else if b.z > far {
            b = interpolar_plano_z(b, a, far);
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

    pub fn clipar_poligono(&self, vertices: Vec<Vec3<f32>>) -> Vec<Vec3<f32>> {
        vec![]
    }
}
