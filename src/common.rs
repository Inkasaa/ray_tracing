use rand::Rng;
 
// Constants

use color::Color;
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use crate::color;
 
// Utility functions
 
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub fn random_double() -> f64 {
    rand::rng().random::<f64>() // Generates a random f64 in the range [0.0, 1.0)
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Return a random real in [min, max)
    min + (max - min) * random_double()
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

#[derive(Clone, Copy)]
pub struct Albedo {
    pub color: Color,
    pub is_spots: bool,
}



pub fn  random_billiard_color(count: usize) -> Albedo {
  
        let base_colors = vec![
            Color::new(0.85, 0.30, 0.04), // Yellow
            Color::new(0.05, 0.04, 0.14), // Blue
            Color::new(0.70, 0.08, 0.03), // Red
            Color::new(0.13, 0.05, 0.11), // Purple
            Color::new(0.85, 0.17, 0.05), // Orange
            Color::new(0.03, 0.12, 0.05), // Green
            Color::new(0.22, 0.04, 0.03), // Maroon
            Color::new(0.02, 0.02, 0.02), // Black
            Color::new(0.72, 0.50, 0.35), // White
        ];


    if count <= 8 {
        Albedo { color: base_colors[count], is_spots: false }
    } else {
        Albedo { color: base_colors[count - 8], is_spots: true }
    }

}