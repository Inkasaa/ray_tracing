use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Box {
    min: Point3,
    max: Point3,
    mat: Rc<dyn Material>,
}

impl Box {
    pub fn new(min: Point3, max: Point3, mat: Rc<dyn Material>) -> Self {
        Self { min, max, mat }
    }

    // Helper to determine which face was hit to calculate the normal
    fn get_normal(&self, p: Point3) -> Vec3 {
        const EPS: f64 = 1e-4;
        if (p.x() - self.min.x()).abs() < EPS {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (p.x() - self.max.x()).abs() < EPS {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (p.y() - self.min.y()).abs() < EPS {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (p.y() - self.max.y()).abs() < EPS {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (p.z() - self.min.z()).abs() < EPS {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }
}

impl Hittable for Box {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / r.direction().e[a];
            let mut t0 = (self.min.e[a] - r.origin().e[a]) * inv_d;
            let mut t1 = (self.max.e[a] - r.origin().e[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }

        rec.t = tmin;
        rec.p = r.at(rec.t);
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(r, self.get_normal(rec.p));
        true
    }
}