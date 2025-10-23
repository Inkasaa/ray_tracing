use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{common, vec3};
use crate::vec3::Vec3;
 
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
 
pub struct Lambertian {
    albedo: Color,
}
 
impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}
 
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
 
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
 
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }
}
 
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
       pub fn new(a: Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: f.clamp(0.0, 1.0), // ensures fuzz is between 0 and 1
        }
    }
}
 
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
      let reflected = vec3::reflect(r_in.direction().unit_vector(), rec.normal);


let diffuse_dir = rec.normal + vec3::random_unit_vector();
let reflected_dir = reflected + self.fuzz * vec3::random_in_unit_sphere();

// linear blend between diffuse and reflection
let blend = 0.35; // 0.0 = fully diffuse, 1.0 = fully reflective
let scatter_dir = Vec3::lerp(diffuse_dir, reflected_dir, blend);

 *scattered = Ray::new(rec.p, scatter_dir);
 
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}

pub struct BiljardColor {
    color: Vec3,
    is_striped: bool,
}

pub struct Spots {
    albedo: Vec3,
    fuzz: f64,
    center: Vec3,
    spot_dir: Vec3, // random direction for spots
}

impl Spots {
    pub fn new(a: Vec3, f: f64, center: Vec3) -> Spots {
        let main_dir = vec3::random_unit_vector(); // random orientation of spots
        Spots {
            albedo: a,
            fuzz: f.clamp(0.0, 1.0),
            center,
            spot_dir: main_dir,
        }
    }
}
 
impl Material for Spots {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(r_in.direction().unit_vector(), rec.normal);
        let diffuse_dir = rec.normal + vec3::random_unit_vector();
        let reflected_dir = reflected + self.fuzz * vec3::random_in_unit_sphere();

        // blend diffuse and reflection
        let blend = 0.35;
        let scatter_dir = Vec3::lerp(diffuse_dir, reflected_dir, blend);
        *scattered = Ray::new(rec.p, scatter_dir);

        // determine spot colors
        let p = (rec.p - self.center).unit_vector();
        let main_dir = self.spot_dir;
        let opposite_dir = -main_dir;
        let spot_centers = [main_dir, opposite_dir];

         // Start with base ball color
        let mut final_color = self.albedo;

        // Big white/tan spots
        let spot_radius = 0.50;
        for c in spot_centers.iter() {
            let dist = p.dot(&c);
            if dist > (1.0 - spot_radius) {
                 final_color = Color::new(0.72, 0.50, 0.35); // warm white/tan
                break;
            }
        }

// === Small circle between the big ones ===

// Choose perpendicular direction
let up = if main_dir.y().abs() > 0.9 {
    Vec3::new(1.0, 0.0, 0.0)
} else {
    Vec3::new(0.0, 1.0, 0.0)
};
let small_spot_dir = main_dir.cross(&up).unit_vector();

// parameters
let small_spot_radius = 0.08;       // radius of white spot
let outer_rim_thickness = 0.02;     // black rim
let inner_black_ring_radius = 0.015; // radius of inner black "0"
let inner_ring_thickness = 0.01;    // thickness of inner black ring

let dist_mid = p.dot(&small_spot_dir);
let angle_diff = 1.0 - dist_mid.abs();

 // Only modify final_color if inside small spot
        if angle_diff < small_spot_radius {
            // Base small spot color (white)
            let mut spot_color = Color::new(0.72, 0.50, 0.35);

            // Outer black rim
            if angle_diff > (small_spot_radius - outer_rim_thickness) {
                spot_color = Color::new(0.0, 0.0, 0.0);
            }

            // Inner black ring ("0")
            if angle_diff < inner_black_ring_radius && angle_diff > (inner_black_ring_radius - inner_ring_thickness) {
                spot_color = Color::new(0.0, 0.0, 0.0);
            }

            // Layer on top of big spot
            final_color = spot_color;
        }

        *attenuation = final_color;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}



pub struct Dielectric {
    ir: f64, // Index of refraction
}
 
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
 
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}
 
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };


        let unit_direction = r_in.direction().unit_vector(); // normalized ray direction
        let cos_theta = f64::min((-unit_direction).dot(&rec.normal), 1.0);

        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
 
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > common::random_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
 
        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, direction);
        true
    }
}