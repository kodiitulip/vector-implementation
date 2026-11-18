use crate::vector::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    p1: Vec2,
    p2: Vec2,
}

impl Line {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }

    pub fn intersects(self, rhs: Line) -> bool {
        let ab = self.p2 - (self.p1);
        let ac = rhs.p1 - (self.p1);
        let ad = rhs.p2 - (self.p1);

        let s1 = ab.cross(ac);
        let s2 = ab.cross(ad);

        // se s1 e s2 tem o mesmo sinal, C e D estao no mesmo lado de AB.
        // Logo, nao tem interseçao
        if s1 * s2 >= 0.0 {
            return false;
        }

        let cd = rhs.p2 - rhs.p1;
        let ca = self.p1 - rhs.p1;
        let cb = self.p2 - rhs.p1;

        let s3 = cd.cross(ca);
        let s4 = cd.cross(cb);

        // Se s3 e s4 não tem o mesmo sinal, A e B estao no mesmo lado de CD.
        // Logo, ha interseção.
        return s3 * s4 < 0.0;
    }
}
