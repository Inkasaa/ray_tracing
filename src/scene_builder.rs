use std::rc::Rc;

use crate::common::{random_billiard_color, random_double, ground_color};
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Metal, NumberType, Lambertian, LambertianNoise};
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::vec3::{Point3, Vec3};
use crate::vec3;

/// Parse a ball index (0-16) to get its color.
pub fn parse_color_or_count(s: &str) -> Option<(Color, bool)> {
    let s = s.trim();
    
    // Try to parse as a number (ball index 0-16)
    if let Ok(count) = s.parse::<usize>() {
        if count <= 16 {
            let albedo = random_billiard_color(count);
            return Some((albedo.color, albedo.is_spots));
        }
    }
    
    None
}

/// Parse background color: ball index (0-16) or three floats r g b (0.0-1.0)
pub fn parse_background_color(s: &str) -> Option<Color> {
    let s = s.trim();
    
    // Try as ball index first
    if let Ok(count) = s.parse::<usize>() {
        if count <= 16 {
            let albedo = random_billiard_color(count);
            return Some(albedo.color);
        }
    }
    
    None
}

/// Parse number_type from a string: 0 = Line, 1 = Circle.
pub fn parse_number_type(s: &str) -> Option<NumberType> {
    let s = s.trim();
    match s {
        "1" => Some(NumberType::Line),
        "0" => Some(NumberType::Circle),
        _ => None,
    }
}

/// Parse ground color index (0-4) to get ground surface color.
pub fn parse_ground_color(s: &str) -> Option<Color> {
    let s = s.trim();
    
    if let Ok(index) = s.parse::<usize>() {
        if index <= 5{
            return Some(ground_color(index));
        }
    }
    
    None
}

/// Container for custom scene: world and optional background color.
pub struct CustomScene {
    pub world: HittableList,
    pub background_color: Option<Color>,
}

/// Build a custom scene from CLI-style arguments.
/// Format: --ball x y z [number_type] [color] ... [--ground [--noise] [color]] ... [--bg color_or_index]
/// 
/// Ball args:
/// - number_type: optional (0 = Circle, 1 = Line). If not provided, chosen randomly.
/// - color: optional (ball index 0-16). If not provided, chosen randomly.
///
/// Ground args (convenience):
/// - Optional --noise flag after --ground uses Perlin noise texture.
/// - Optional color index 0-9 selects the billiard table felt color:
///   0: Tournament Green (default), 1: Dark Green, 2: English Green,
///   3: Navy Blue, 4: Electric Blue, 5: Burgundy/Wine Red,
///   6: Dark Red, 7: Camel/Tan, 8: Charcoal Grey, 9: Purple
/// - Without --noise, uses solid Lambertian with the specified color.
///   Always creates a horizontal plane at origin (0,0,0) with upward normal (0,1,0).
///
/// Background:
/// - --bg: optional background color (ball index 0-16)
///
/// Examples:
/// --ground --ball 0 0.2 1 1 3               (Default tournament green noise ground)
/// --ground 3 --ball 0 0.2 1 1 3             (Solid navy blue felt)
/// --ground --noise 3 --ball 0 0.2 1 1 3     (Navy blue noise felt)
/// --ground --noise 5 --ball 0 0.2 1 1 3     (Burgundy noise felt)
pub fn build_custom_scene(args: &[String]) -> Option<CustomScene> {
    let mut world = HittableList::new();
    let mut background_color: Option<Color> = None;
    let mut i = 0;

    let mut ground_created = false;
    while i < args.len() {
        if args[i] == "--ball" {
            // Minimum: x y z (3 args after --ball)
            if i + 3 >= args.len() {
                return None;
            }

            let x: f64 = args[i + 1].parse().ok()?;
            let y: f64 = args[i + 2].parse().ok()?;
            let z: f64 = args[i + 3].parse().ok()?;

            let mut consumed = 4; // --ball, x, y, z
            let mut number_type_opt: Option<NumberType> = None;
            let mut color_opt: Option<(Color, bool)> = None;

            // Try to parse next arg as number_type (0 or 1)
            if i + consumed < args.len() {
                if let Some(nt) = parse_number_type(&args[i + consumed]) {
                    number_type_opt = Some(nt);
                    consumed += 1;
                }
            }

            // Try to parse next arg as color (ball index 0-16)
            if i + consumed < args.len() {
                if let Some(c) = parse_color_or_count(&args[i + consumed]) {
                    color_opt = Some(c);
                    consumed += 1;
                }
            }

            // If no color provided, pick one randomly (0-16)
            let (color, is_spots) = if let Some((c, s)) = color_opt {
                (c, s)
            } else {
                let random_idx = (random_double() * 17.0) as usize; // 0..16
                let albedo = random_billiard_color(random_idx);
                (albedo.color, albedo.is_spots)
            };

            // If no number_type provided, pick one randomly (0 or 1)
            let number_type = if let Some(nt) = number_type_opt {
                nt
            } else {
                if random_double() < 0.5 {
                    NumberType::Line
                } else {
                    NumberType::Circle
                }
            };

            let center = Point3::new(x, y, z);
            let fuzz = 0.01;
            let spot_dir = vec3::random_unit_vector();

            let ball_material: Rc<dyn crate::material::Material> = if is_spots {
                Rc::new(crate::material::Striped::new(color, fuzz, center, number_type))
            } else {
                Rc::new(Metal::new(color, fuzz, center, spot_dir, number_type))
            };

            world.add(Box::new(Sphere::new(center, 0.2, ball_material)));

            i += consumed;
        } else if args[i] == "--plane" {
            // Deprecated: ignore legacy --plane flag
            i += 1; // Skip the flag; any following numbers will be treated as other flags/args
        } else if args[i] == "--ground" {
            // Ground format: --ground [--noise] [color_index]
            // Only one ground plane allowed; ignore subsequent flags.
            if ground_created {
                i += 1; // skip duplicate
                continue;
            }

            let mut consumed = 1; // --ground
            let ground_point = Point3::new(0.0, 0.0, 0.0);
            let ground_normal = Vec3::new(0.0, 1.0, 0.0);

            // Check for --noise flag
            let use_noise = if i + consumed < args.len() && args[i + consumed] == "--noise" {
                consumed += 1;
                true
            } else {
                false
            };

            // Check for ground color index (0-9, optional)
            let ground_color_value: Color = if i + consumed < args.len() {
                if let Some(c) = parse_ground_color(&args[i + consumed]) {
                    consumed += 1;
                    c
                } else {
                    ground_color(0) // Default to index 0 (dark green grass)
                }
            } else {
                ground_color(0) // Default to index 0 (dark green grass)
            };

            // Create material: noise if no --noise flag AND no color index, or if --noise flag present
            let ground_mat: Rc<dyn crate::material::Material> = if use_noise {
                Rc::new(LambertianNoise::new(ground_color_value, 200.0))
            } else if i == consumed { // No --noise, no color index = default noise
                Rc::new(LambertianNoise::new(ground_color_value, 200.0))
            } else {
                Rc::new(Lambertian::new(ground_color_value))
            };

            world.add(Box::new(Plane::new(ground_point, ground_normal, ground_mat)));
            ground_created = true;
            i += consumed;
        } else if args[i] == "--bg" {
            // Background color: --bg color_index
            if i + 1 >= args.len() {
                return None;
            }
            background_color = parse_background_color(&args[i + 1]);
            i += 2;
        } else {
            i += 1;
        }
    }

    Some(CustomScene {
        world,
        background_color,
    })
}

