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

      pub       struct Albedo {
              pub  color: Color,
              pub   is_spots: bool,
}
     
use rand::seq::SliceRandom;
use rand::rng;

pub struct BilliardColorPicker {
    colors: Vec<Color>,
    current: usize,
}

impl BilliardColorPicker {
    pub fn new() -> Self {
        let mut colors = vec![
            Color::new(0.85, 0.30, 0.04), // 1 - Yellow
            Color::new(0.05, 0.04, 0.14), // 2 - Blue
            Color::new(0.70, 0.08, 0.03), // 3 - Red
            Color::new(0.13, 0.05, 0.11), // 4 - Purple
            Color::new(0.85, 0.17, 0.05), // 5 - Orange
            Color::new(0.03, 0.12, 0.05), // 6 - Green
            Color::new(0.22, 0.04, 0.03), // 7 - Maroon
            Color::new(0.02, 0.02, 0.02), // 8 - Black
            Color::new(0.72, 0.50, 0.35), // 9 - White
        ];
        // Shuffle the colors to get a random order
        colors.shuffle(&mut rng());
        Self {
            colors,
            current: 0,
        }
    }

    pub fn random_billiard_color(&mut self) -> Option<Albedo> {
        if self.current >= self.colors.len() {
            return None; // all colors have been used
        }

        let color = self.colors[self.current];
        self.current += 1;

        let is_spots = self.current > 8; // keep your original logic

        Some(Albedo { color, is_spots,})
    }
}

