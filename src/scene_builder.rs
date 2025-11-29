use std::rc::Rc;

use crate::common::{random_billiard_color, random_double};
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Metal, NumberType, Lambertian};
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

/// Container for custom scene: world and optional background color.
pub struct CustomScene {
    pub world: HittableList,
    pub background_color: Option<Color>,
}

/// Build a custom scene from CLI-style arguments.
/// Format: --ball x y z [number_type] [color] ... [--plane px py pz nx ny nz color] ... [--bg color_or_index]
/// 
/// Ball args:
/// - number_type: optional (0 = Circle, 1 = Line). If not provided, chosen randomly.
/// - color: optional (ball index 0-16). If not provided, chosen randomly.
///
/// Plane args:
/// - px py pz: point on the plane
/// - nx ny nz: normal vector (will be normalized)
/// - color: ball index 0-16
///
/// Background:
/// - --bg: optional background color (ball index 0-16)
///
/// Examples:
/// --ball 1.0 0.2 2.0 0 5 --bg 16              (Yellow Line ball + white background)
/// --plane 0 0 0 0 1 0 7 --ball 0 0.2 1 1 3    (Maroon ground plane + circle green ball)
/// --plane 0 0 0 0 1 0 7 --ball 0 0.2 1 1 3 --bg 1  (Same with blue background)
pub fn build_custom_scene(args: &[String]) -> Option<CustomScene> {
    let mut world = HittableList::new();
    let mut background_color: Option<Color> = None;
    let mut i = 0;

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
            // Plane format: --plane px py pz nx ny nz color_index
            // Minimum: 7 args after --plane
            if i + 7 >= args.len() {
                return None;
            }

            let px: f64 = args[i + 1].parse().ok()?;
            let py: f64 = args[i + 2].parse().ok()?;
            let pz: f64 = args[i + 3].parse().ok()?;
            let nx: f64 = args[i + 4].parse().ok()?;
            let ny: f64 = args[i + 5].parse().ok()?;
            let nz: f64 = args[i + 6].parse().ok()?;

            let plane_point = Point3::new(px, py, pz);
            let plane_normal = Vec3::new(nx, ny, nz);

            let (plane_color, _) = parse_color_or_count(&args[i + 7])?;

            let plane_material: Rc<dyn crate::material::Material> =
                Rc::new(Lambertian::new(plane_color));

            world.add(Box::new(Plane::new(plane_point, plane_normal, plane_material)));

            i += 8;
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

