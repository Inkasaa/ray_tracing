use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Cylinder {
    p1: Point3, // Center of one end cap
    p2: Point3, // Center of the other end cap
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(p1: Point3, p2: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self { p1, p2, radius, mat }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let axis = self.p2 - self.p1;
        let height = axis.length();
        let axis_norm = axis / height;

        let oc = r.origin() - self.p1;

        let card = axis_norm.dot(&r.direction());
        let caoc = axis_norm.dot(&oc);

        let a = r.direction().length_squared() - card * card;
        let b = 2.0 * (r.direction().dot(&oc) - card * caoc);
        let c = oc.length_squared() - caoc * caoc - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let t = (-b - sqrt_d) / (2.0 * a);

        if t > t_min && t < t_max {
            let p = r.at(t);
            let proj = (p - self.p1).dot(&axis_norm);

            if proj >= 0.0 && proj <= height {
                rec.t = t;
                rec.p = p;
                let outward_normal = (p - self.p1 - proj * axis_norm) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.mat.clone());
                return true;
            }
        }

        let t2 = (-b + sqrt_d) / (2.0 * a);
        if t2 > t_min && t2 < t_max {
            let p = r.at(t2);
            let proj = (p - self.p1).dot(&axis_norm);

            if proj >= 0.0 && proj <= height {
                rec.t = t2;
                rec.p = p;
                let outward_normal = (p - self.p1 - proj * axis_norm) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.mat.clone());
                return true;
            }
        }

        // Note: This implementation does not include end caps for simplicity.
        false
    }
}