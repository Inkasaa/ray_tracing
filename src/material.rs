use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{common, vec3};
use crate::vec3::{Point3, Vec3};
 
// --- Constants for Billiard Ball Spot Rendering ---
const BILLIARD_SPOT_WHITE: Color = Color::new(0.72, 0.50, 0.35); // Warm white/tan for spots
const BILLIARD_SPOT_BLACK: Color = Color::new(0.0, 0.0, 0.0);

#[derive(Clone, Copy)]
pub enum NumberType {
    Circle, // "0"
    Line,   // "1"
}

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
   pub albedo: Color,
   pub fuzz: f64,
   pub center: Vec3,
   pub spot_dir: Vec3,
   pub number_type: NumberType,
}

impl Metal {
       pub fn new(a: Color, f: f64, center: Vec3, spot_dir: Vec3, number_type: NumberType) -> Metal {
        Metal {
            albedo: a,
            fuzz: f.clamp(0.0, 1.0), // ensures fuzz is between 0 and 1
            center: center,
            spot_dir: spot_dir.unit_vector(),
            number_type,
        }
    }

    /// Calculates the color for a solid-colored ball with a single numbered spot (like the 8-ball).
    fn get_solid_ball_color(&self, hit_point: &Point3) -> Color {
        // If the ball is the white cue ball, don't draw a numbered spot on it.
        if (self.albedo - BILLIARD_SPOT_WHITE).near_zero() {
            return self.albedo;
        }

        let p_normalized = (*hit_point - self.center).unit_vector();
        let alignment = p_normalized.dot(&self.spot_dir);

        // Check if the hit point is within the small spot area
        if (1.0 - alignment) < 0.08 { // small_spot_radius
            return get_number_spot_color(p_normalized, self.spot_dir, self.number_type, None);
        }

        self.albedo
    }
}

/// Draws a small, numbered white spot with a black ring.
fn get_number_spot_color(p_normalized: Vec3, spot_center_dir: Vec3, number_type: NumberType, pole_dir: Option<Vec3>) -> Color {
    let alignment = p_normalized.dot(&spot_center_dir);
    let angle_diff = 1.0 - alignment;

    // Parameters controlling the look of the spot
    const NBR_SPOT_RADIUS: f64 = 0.08;
    const OUTER_BLACK_RIM_THICKNESS: f64 = 0.0135;
    const OUTER_WHITE_RING_THICKNESS: f64 = 0.0135; // This creates the white ring at the edge.
    const LINE_THICKNESS: f64 = 0.05; // Increased from 0.02 to make the line thicker
    const CIRCLE_THICKNESS: f64 = 0.015; 
    const CIRCLE_RADIUS: f64 = 0.02; 

    // Determine color based on position within the spot
    // The black rim is now inset from the edge to leave a white ring.
    if angle_diff > (NBR_SPOT_RADIUS - OUTER_BLACK_RIM_THICKNESS - OUTER_WHITE_RING_THICKNESS) 
    && angle_diff < (NBR_SPOT_RADIUS - OUTER_WHITE_RING_THICKNESS) 
    { BILLIARD_SPOT_BLACK } // Outer black rim
    else {
        // Logic for the number inside the spot
        match number_type {
            NumberType::Circle => {
                // Draw a circle with a radius of 0.02
                if angle_diff < CIRCLE_RADIUS && angle_diff > (CIRCLE_RADIUS - CIRCLE_THICKNESS) {
                    BILLIARD_SPOT_BLACK // Inner black ring ("0")
                } else {
                    BILLIARD_SPOT_WHITE // Center white part
                }
            }
            NumberType::Line => {
                // Find a perpendicular "up" vector to draw the line
                // If a pole direction is given (for striped balls), use it to define "up".
                // Otherwise, use an arbitrary "up" vector.
                let vertical_axis = pole_dir.unwrap_or_else(|| {
                    if spot_center_dir.y().abs() > 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) }
                });

                let horizontal_dir = spot_center_dir.cross(&vertical_axis).unit_vector();
                let vertical_dir = spot_center_dir.cross(&horizontal_dir); // This is perpendicular to the line
                
                let dot_horizontal = p_normalized.dot(&horizontal_dir);
                let dot_vertical = p_normalized.dot(&vertical_dir);

                // The inner circle for "0" has a radius of 0.02.
                // To make the line for "1" have the same height, its half-length should match this radius.
                const LINE_HALF_LENGTH: f64 = 0.2;

                // Check if the point is within a thick vertical band of limited length
                if dot_horizontal.abs() < LINE_THICKNESS && dot_vertical.abs() < LINE_HALF_LENGTH { BILLIARD_SPOT_BLACK } else { BILLIARD_SPOT_WHITE }
            }
        }
    }
}
 
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        const SCATTER_BLEND: f64 = 0.35;
        let scatter_dir = calculate_mixed_scatter_direction(r_in, rec, self.fuzz, SCATTER_BLEND);
        *scattered = Ray::new(rec.p, scatter_dir); 
        *attenuation = self.get_solid_ball_color(&rec.p);
        
        scattered.direction().dot(&rec.normal) > 0.0
    }

}

pub struct Striped {
    albedo: Color,
    fuzz: f64,
    center: Vec3,
    spot_dir: Vec3, // random direction for spots
    number_type: NumberType,
}

impl Striped {
    pub fn new(a: Color, f: f64, center: Vec3, number_type: NumberType) -> Striped {
        let main_dir = vec3::random_unit_vector(); // random orientation of spots
        Striped {
            albedo: a,
            fuzz: f.clamp(0.0, 1.0),
            center,
            spot_dir: main_dir,
            number_type,
        }
    }

    /// Calculates the color for a striped ball with two large spots and a numbered spot.
    fn get_striped_ball_color(&self, hit_point: &Point3) -> Color {
        let p = (*hit_point - self.center).unit_vector();
        
        // --- Big white/tan spots on opposite ends ---
        const BIG_SPOT_RADIUS: f64 = 0.50;
        let opposite_dir = -self.spot_dir;
        let spot_centers = [self.spot_dir, opposite_dir];

        for c in spot_centers.iter() {
            if p.dot(c) > (1.0 - BIG_SPOT_RADIUS) {
                return BILLIARD_SPOT_WHITE;
            }
        }

        // --- Small "number" spot between the big spots ---
        // Find a direction perpendicular to the main spot axis
        let up = if self.spot_dir.y().abs() > 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let small_spot_center_dir = self.spot_dir.cross(&up).unit_vector();

        if (1.0 - p.dot(&small_spot_center_dir)) < 0.08 {
            return get_number_spot_color(p, small_spot_center_dir, self.number_type, Some(self.spot_dir));
        }

        // If not in any spot, return the base stripe color
        self.albedo
    }
}

/// Helper to calculate a blended scatter direction between diffuse and reflective.
fn calculate_mixed_scatter_direction(r_in: &Ray, rec: &HitRecord, fuzz: f64, blend: f64) -> Vec3 {
    let reflected = vec3::reflect(r_in.direction().unit_vector(), rec.normal);
    let diffuse_dir = rec.normal + vec3::random_unit_vector();
    let reflected_dir = reflected + fuzz * vec3::random_in_unit_sphere();
    Vec3::lerp(diffuse_dir, reflected_dir, blend)
}
 
impl Material for Striped {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        const SCATTER_BLEND: f64 = 0.35;
        let scatter_dir = calculate_mixed_scatter_direction(r_in, rec, self.fuzz, SCATTER_BLEND);
        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.get_striped_ball_color(&rec.p);
        
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
