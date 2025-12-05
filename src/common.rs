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
   
         
            Color::new(0.02, 0.02, 0.02), // Black
            Color::new(0.05, 0.04, 0.14), // Blue
            Color::new(0.13, 0.05, 0.11), // Purple
            Color::new(0.03, 0.12, 0.05), // Green
            Color::new(0.85, 0.17, 0.05), // Orange
            Color::new(0.85, 0.30, 0.04), // Yellow
            Color::new(0.73, 0.07, 0.03), // Red
            Color::new(0.22, 0.04, 0.03), // Maroon
         
            //same colors but in different order for stripes
   
            Color::new(0.85, 0.30, 0.04), // Yellow
            Color::new(0.05, 0.04, 0.14), // Blue
            Color::new(0.73, 0.07, 0.03), // Red
            Color::new(0.13, 0.05, 0.11), // Purple
            Color::new(0.85, 0.17, 0.05), // Orange
            Color::new(0.03, 0.12, 0.05), // Green
            Color::new(0.22, 0.04, 0.03), // Maroon
            Color::new(0.02, 0.02, 0.02), // Black

            Color::new(0.72, 0.50, 0.35), // White
        ];


    if count <= 7 || count == 16{
        Albedo { color: base_colors[count], is_spots: false }
    } else if count < 16{
        Albedo { color: base_colors[count], is_spots: true }
    } else {
       let  new_count = count%16;
        Albedo { color: base_colors[new_count], is_spots: false }
    
    }

}

/// Parse ball color name to index (0-16)
/// Returns Some(index) if valid color name, None otherwise
pub fn ball_color_name_to_index(name: &str) -> Option<usize> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        // Solid colors (0-7)
        "black" => Some(0),
        "blue" => Some(1),
        "purple" => Some(2),
        "green" => Some(3),
        "orange" => Some(4),
        "yellow" => Some(5),
        "red" => Some(6),
        "maroon" => Some(7),
        // Striped/spots variants (8-15) - using "color2" format
        "yellow2" => Some(8),
        "blue2" => Some(9),
        "red2" => Some(10),
        "purple2" => Some(11),
        "orange2" => Some(12),
        "green2" => Some(13),
        "maroon2" => Some(14),
        "black2" => Some(15),
        // Cue ball (16)
        "white" | "cue" | "cue-ball" | "cue_ball" => Some(16),
        _ => None,
    }
}

/// Ground color palette - billiard table felt colors
/// Index 0-4 for different felt colors commonly used on pool tables
pub fn ground_color(index: usize) -> Color {
    let ground_colors = vec![
        Color::new(0.099, 0.172, 0.095), // 0: Tournament Green (default, same as random scene)
        Color::new(0.099, 0.172, 0.195),    // 1: Electric Blue
        Color::new(0.36, 0.06, 0.07),    // 2: Burgundy/Wine Red
        Color::new(0.09, 0.3, 0.11),    // 3: PAF green
        Color::new(0.038, 0.088, 0.068),    // 4: PAF dark 
         Color::new(0.012, 0.043, 0.032),    // 5: PAF dark smooth
    ];
    //rgb01(0.33, 0.61, 0.64)

    if index < ground_colors.len() {
        ground_colors[index]
    } else {
        ground_colors[0] // Default to tournament green
    }
}