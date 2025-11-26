use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Plane {
    point: Point3,
    normal: Vec3,
    mat: Rc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, mat: Rc<dyn Material>) -> Self {
        Self {
            point,
            normal: normal.unit_vector(),
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&r.direction());

        // If the ray is parallel to the plane, it doesn't intersect.
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.point - r.origin()).dot(&self.normal) / denom;

        if t <= t_min || t >= t_max {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);

        true
    }
}