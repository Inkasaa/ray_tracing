use std::rc::Rc;

use crate::common::{random_billiard_color, random_double, ground_color, ball_color_name_to_index};
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Metal, NumberType, Lambertian, LambertianNoise};
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::vec3::{Point3, Vec3};
use crate::vec3;

/// Map ground felt texture names to indices (0-5).
/// Accepted names (case-insensitive):
/// - "Tournament Green" (0)
/// - "Electric Blue" (1)
/// - "Burgundy" or "Wine Red" (2)
/// - "PAF green" (3)
/// - "PAF dark" (4)
/// - "PAF dark smooth" (5)
fn ground_texture_name_to_index(name: &str) -> Option<usize> {
    let n = name.trim().to_lowercase();
    match n.as_str() {
        "green" => Some(0),
        "blue" => Some(1),
        "burgundy" | "red" => Some(2),
        "paf green" => Some(3),
        "paf dark" => Some(4),
        "paf darker" => Some(5),
        _ => None,
    }
}

/// Parse a ball color index (0-16) or color name to get its color.
/// Accepts numbers (0-16) or names like "black", "blue", "yellow-stripe", "white", etc.
pub fn parse_color_or_count(s: &str) -> Option<(Color, bool)> {
    let s = s.trim();
    
    // Try to parse as a number (ball index 0-16)
    if let Ok(count) = s.parse::<usize>() {
        if count <= 16 {
            let albedo = random_billiard_color(count);
            return Some((albedo.color, albedo.is_spots));
        }
    }
    
    // Try to parse as a color name
    if let Some(index) = ball_color_name_to_index(s) {
        let albedo = random_billiard_color(index);
        return Some((albedo.color, albedo.is_spots));
    }
    
    None
}

/// Parse background color: ball index (0-16), color name, or three floats r g b (0.0-1.0)
pub fn parse_background_color(s: &str) -> Option<Color> {
    let s = s.trim();
    
    // Try as ball index first
    if let Ok(count) = s.parse::<usize>() {
        if count <= 16 {
            let albedo = random_billiard_color(count);
            return Some(albedo.color);
        }
    }
    
    // Try as color name
    if let Some(index) = ball_color_name_to_index(s) {
        let albedo = random_billiard_color(index);
        return Some(albedo.color);
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
/// Format: --ball x y z color [number_type] [yaw pitch] ... [--ground [--noise] [color]] ... [--bg color_or_index]
/// 
/// Ball args:
/// - color: REQUIRED (ball index 0-16 OR color name).
///   Color names: black, blue, purple, green, orange, yellow, red, maroon, white
///   Striped variants: yellow2, blue2, red2, purple2, orange2, green2, maroon2, black2
/// - number_type: optional (0 = Circle, 1 = Line). If not provided, chosen randomly.
/// - yaw, pitch: optional angles in degrees (yaw = rotation around Y axis, pitch = rotation from horizontal). If not provided, chosen randomly.
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
/// - --bg: optional background color (ball index 0-16 OR color name)
///
/// Examples:
/// --ground --ball 0 0.2 1 1 yellow               (Default tournament green, yellow circle ball)
/// --ground 3 --ball 0 0.2 1 1 red                (Solid navy blue felt, red circle ball)
/// --ground --noise 3 --ball 0 0.2 1 0 blue2      (Navy blue noise felt, striped blue line ball)
/// --ground --noise 5 --ball 0 0.2 1 1 white --bg maroon (Burgundy noise felt, white ball, maroon bg)
pub fn build_custom_scene(args: &[String]) -> Option<CustomScene> {
    let mut world = HittableList::new();
    let mut background_color: Option<Color> = None;
    let mut i = 0;

    let mut ground_created = false;
    while i < args.len() {
        if args[i] == "--ball" {
                // Required: x y z color (4 args after --ball)
                if i + 4 >= args.len() {
                return None;
            }

            let x: f64 = args[i + 1].parse().ok()?;
            let y: f64 = args[i + 2].parse().ok()?;
            let z: f64 = args[i + 3].parse().ok()?;

                // Parse color (REQUIRED) - can be index 0-16 or name like "purple2"
                let (color, is_spots) = parse_color_or_count(&args[i + 4])?;
            
                let mut consumed = 5; // --ball, x, y, z, color

                // Try to parse next arg as optional number_type (0 or 1)
                let number_type = if i + consumed < args.len() {
                    if let Some(nt) = parse_number_type(&args[i + consumed]) {
                        consumed += 1;
                        nt
                    } else {
                        if random_double() < 0.5 { NumberType::Line } else { NumberType::Circle }
                    }
                } else if random_double() < 0.5 { NumberType::Line } else { NumberType::Circle };

                // Optional yaw and pitch angles after number_type or color
                let mut orientation: Option<Vec3> = None;
                if i + consumed + 1 < args.len() {
                    let maybe_yaw = args[i + consumed].parse::<f64>();
                    let maybe_pitch = args[i + consumed + 1].parse::<f64>();
                    if let (Ok(yaw_deg), Ok(pitch_deg)) = (maybe_yaw, maybe_pitch) {
                        // Convert degrees to radians
                        let yaw = yaw_deg.to_radians();
                        let pitch = pitch_deg.to_radians();
                        
                        // Convert spherical coordinates (yaw, pitch) to Cartesian vector
                        // yaw = rotation around vertical (Y) axis
                        // pitch = rotation from horizontal plane
                        let x = pitch.cos() * yaw.sin();
                        let y = pitch.sin();
                        let z = pitch.cos() * yaw.cos();
                        
                        orientation = Some(Vec3::new(x, y, z));
                        consumed += 2;
                    }
                }

            let center = Point3::new(x, y, z);
            let fuzz = 0.01;
            let spot_dir = orientation.unwrap_or_else(|| vec3::random_unit_vector()).unit_vector();

            let ball_material: Rc<dyn crate::material::Material> = if is_spots {
                if let Some(dir) = orientation { Rc::new(crate::material::Striped::new_with_dir(color, fuzz, center, number_type, dir)) }
                else { Rc::new(crate::material::Striped::new(color, fuzz, center, number_type)) }
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
        } else if args[i] == "--table" {
            // New convenience: --table [--texture <felt name>]
            // Creates ground plane; if --texture is provided, enable Perlin noise using the named felt color.
            // Examples:
            //   --table --texture PAF green
            //   --table --texture Electric Blue
            if ground_created {
                i += 1;
                continue;
            }

            let mut consumed = 1; // --table
            let ground_point = Point3::new(0.0, 0.0, 0.0);
            let ground_normal = Vec3::new(0.0, 1.0, 0.0);

            // Defaults
            let mut use_noise = false;
            let mut felt_color = ground_color(0); // Tournament Green

            // Parse optional --texture <name>
            if i + consumed < args.len() && args[i + consumed] == "--texture" {
                consumed += 1; // consume --texture
                if i + consumed < args.len() {
                    // Allow two-word names like "PAF green"
                    let name1 = args[i + consumed].clone();
                    let mut texture_name = name1.clone();
                    consumed += 1;
                    if i + consumed < args.len() {
                        let next = &args[i + consumed];
                        // If next token is not a flag, treat as second word
                        if !next.starts_with("--") {
                            texture_name = format!("{} {}", name1, next);
                            consumed += 1;
                        }
                    }
                    if let Some(idx) = ground_texture_name_to_index(&texture_name) {
                        felt_color = ground_color(idx);
                        use_noise = true; // texture implies noise felt
                    }
                }
            }

            let ground_mat: Rc<dyn crate::material::Material> = if use_noise {
                Rc::new(LambertianNoise::new(felt_color, 200.0))
            } else {
                Rc::new(Lambertian::new(felt_color))
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

