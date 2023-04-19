use super::*;

pub struct ONB {
    axis: [Vector3; 3],
}

// TBN Space
impl ONB {
    pub fn build_from_w(n: &Vector3) -> ONB {
        let w = n.normalize();
        let a = if w.x().abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);

        ONB { axis: [u, v, w] }
    }

    pub fn u(&self) -> Vector3{
        self.axis[0]
    }

    pub fn v(&self) -> Vector3{
        self.axis[1]
    }
    pub fn w(&self) -> Vector3{
        self.axis[2]
    }

    pub fn local(&self, a: &Vector3) -> Vector3{
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}

