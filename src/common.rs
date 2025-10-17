use rand::Rng;
 
// Constants

use color::Color;
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use crate::{color, common};
 
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

pub fn random_billiard_color() -> Color {
    let colors = [
    Color::new(0.85, 0.45, 0.05), // 1 - Yellow
    Color::new(0.03, 0.08, 0.15), // 2 - Blue
    Color::new(0.75, 0.05, 0.05), // 3 - Red
    Color::new(0.10, 0.05, 0.10), // 4 - Purple
    Color::new(0.85, 0.2, 0.05), // 5 - Orange
    Color::new(0.03, 0.12, 0.05), // 6 - Green
   Color::new(0.25, 0.05, 0.05), // 7 - Maroon
    Color::new(0.02, 0.02, 0.02), // 8 - Black
    Color::new(0.92, 0.92, 0.92), // 9 - white
    ];
    let index = (common::random_double() * colors.len() as f64) as usize;
    colors[index.min(colors.len() - 1)]
}
