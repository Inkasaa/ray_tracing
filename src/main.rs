mod camera;
mod color;
mod common;
mod cuboid;
mod cylinder;
mod hittable;
mod hittable_list;
mod material;
mod plane;
mod ray;
mod sphere;
mod vec3;
mod light;
mod scene_builder;
use crate::{cuboid::Cuboid, cylinder::Cylinder, light::{compute_light, PointLight}, material::{Material, NumberType, Striped}, plane::Plane, vec3::{Point3, Vec3}};
use crate::scene_builder::build_custom_scene;
 
use std::{env, fs::File, io::{BufWriter, Write}};
use std::rc::Rc;

use common::*;
use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList; 
use material::{Lambertian, Metal, LambertianNoise};
use ray::Ray;
use sphere::Sphere;
 
fn ray_color(r: &Ray, world: &dyn Hittable, lights: &[PointLight], depth: i32, bg_color: Option<Color>) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
 
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
            let view_dir = (-r.direction()).unit_vector();

            // --- 1. Calculate Indirect (scattered) light ---
            let mut attenuation = Color::default();
            let mut scattered = Ray::default();
            let indirect_light = if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    attenuation * ray_color(&scattered, world, lights, depth - 1, bg_color)
                } else {
                    Color::new(0.0, 0.0, 0.0)
                }
            } else {
                Color::new(0.0, 0.0, 0.0)
            };
        
           // --- 2. Calculate Direct light from all light sources ---
            let mut direct_light = Color::new(0.0, 0.0, 0.0);
            for light in lights.iter() {
                const SAMPLES: i32 = 4; // Use a constant for shadow samples
                let mut light_contribution = Color::new(0.0, 0.0, 0.0);

                for _ in 0..SAMPLES {
                    let light_radius = 1.4;
                    let jitter = vec3::random_in_unit_sphere() * light_radius;
                    let sample_pos = light.position + jitter;
                    let to_light = sample_pos - rec.p;
                    let light_dist = to_light.length();
                    let light_dir = to_light / light_dist;
                    let shadow_ray = Ray::new(rec.p + rec.normal * 0.001, light_dir);

                    if !world.hit(&shadow_ray, 0.001, light_dist - 0.001, &mut HitRecord::new()) {
                        // If the material pattern is black, only add specular highlights.
                        if attenuation.near_zero() {
                            light_contribution += compute_light(rec.p, rec.normal, view_dir, Color::new(0.0, 0.0, 0.0), light).1; // Specular only
                        } else {
                            let (diffuse, specular) = compute_light(rec.p, rec.normal, view_dir, attenuation, light);
                            light_contribution += diffuse + specular;
                        }
                    }
                }
                direct_light += light_contribution / SAMPLES as f64;
            }

            // --- 3. Combine and return final color ---
            return indirect_light + direct_light;
    }

    // Background: use custom color if provided, otherwise use gradient
    if let Some(custom_bg) = bg_color {
        // Use solid custom background color (no gradient)
        custom_bg
    } else {
        // Default gradient: white to light blue
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
/* 
//fn a_sphere() -> HittableList {
//    let mut world = HittableList::new();
//    let center = Point3::new(
//                    a as f64 + 0.7 * common::random_double(),
//                    0.2,
//                    b as f64 + 0.7 * common::random_double(),
                );

                let color = random_billiard_color(count_balls);

                let fuzz = 0.01;
                let spot_dir = vec3::random_unit_vector();

                let number_type = if count_balls % 2 == 0 {
                    NumberType::Line
                } else {
                    NumberType::Circle
                };

                let sphere_material: Rc<dyn Material> = if color.is_spots {
                    Rc::new(Striped::new(color.color, fuzz, center, number_type))
                } else {
                    Rc::new(Metal::new(color.color, fuzz, center, spot_dir, number_type))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));

                count_balls += 1;
            }
        }
    world
}
*/ 
fn random_scene() -> HittableList {
    let mut world = HittableList::new();
 
    // Ground material with a subtle Perlin noise texture (marble-like)
    // Increased scale for finer (smaller) features
    let ground_material = Rc::new(LambertianNoise::new(Color::new(0.099, 0.172, 0.095), 200.0)); 
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),    // A point on the plane (the origin)
        Vec3::new(0.0, 1.0, 0.0),      // The normal vector (pointing straight up)
        ground_material,
    )));

    // Add a small cube to the scene
    let box_material = Rc::new(Lambertian::new(Color::new(0.12, 0.20, 0.15))); // Muted green
    world.add(Box::new(Cuboid::new(
        Point3::new(1.5, 0.0, 3.0), //X = Width, Y = Height, Z = Depth
        Point3::new(1.7, 0.2, 3.2),
        box_material,
    )));

    // Add a cylinder to the scene
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.3, 0.15, 0.05))); // Brown
    world.add(Box::new(Cylinder::new(
        Point3::new(1.65, 0.23, 3.1), // starting point
        Point3::new(4.0, 1.0, -0.10), // ending point (towards camera)
        0.04, // Thin radius
        cylinder_material,
    )));
 
 let mut count_balls = 0;
 let nbr_of_balls = 16;
    for a in -1..3 {
        for b in -1..4 {
            if count_balls <= nbr_of_balls {
                let center = Point3::new(
                    a as f64 + 0.7 * common::random_double(),
                    0.2,
                    b as f64 + 0.7 * common::random_double(),
                );

                let color = random_billiard_color(count_balls);

                let fuzz = 0.01;
                let spot_dir = vec3::random_unit_vector();

                let number_type = if count_balls % 2 == 0 {
                    NumberType::Line
                } else {
                    NumberType::Circle
                };

                let sphere_material: Rc<dyn Material> = if color.is_spots {
                    Rc::new(Striped::new(color.color, fuzz, center, number_type))
                } else {
                    Rc::new(Metal::new(color.color, fuzz, center, spot_dir, number_type))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));

                count_balls += 1;
            }
        }
    }
    world
}
 
fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 200;
    const MAX_DEPTH: i32 = 100;
 
    // Animation
    const ROTATION_DEGREES: f64 = 250.0; // Full 360-degree rotation

    // Get number of frames from command-line argument, with a default value.
    let args: Vec<String> = env::args().collect();
    let num_frames = if args.len() > 1 {
        args[1].parse().unwrap_or(60)
    } else {
        60 // Default number of frames
    };

    // World and background color: use custom scene if --custom flag is present, otherwise use random_scene
    let (world, custom_bg_color) = if args.iter().any(|s| s == "--custom") {
        match build_custom_scene(&args) {
            Some(custom_scene) => {
                eprintln!("Using custom billiard ball scene from CLI arguments");
                (custom_scene.world, custom_scene.background_color)
            }
            None => {
                eprintln!("Failed to parse custom scene, falling back to random_scene");
                (random_scene(), None)
            }
        }
    } else {
        (random_scene(), None)
    };
    if let Some(bg) = custom_bg_color {
        eprintln!("DEBUG: custom_bg_color = {:?}", bg);
    } else {
        eprintln!("DEBUG: custom_bg_color = None (using default gradient)");
    }
 
    let intensity = 0.75;
    let lights = vec![
        PointLight::new(Point3::new(0.0, 6.0, 4.0), Color::new(1.0, 0.85, 0.6), intensity), // overhead lamp
    ];

    // --- Render Loop for Video ---
    for frame in 0..num_frames {
        // --- Calculate Camera Position for this frame ---
        let lookat = Point3::new(0.85, 0.2, -1.0);
        let vup = Point3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 6.0;
        let aperture = 0.02; //0.0 pinhole 0.05 noticably blur

        // Orbit parameters
        let radius = 14.5 - ( 0.17 * frame as f64); // Distance from lookat point in the XZ plane
        let start_angle_rad = 0.46; // Initial angle to match the original view
        let angle_step = degrees_to_radians(ROTATION_DEGREES) / num_frames as f64;
        let current_angle = start_angle_rad + frame as f64 * angle_step;

        let lookfrom = Point3::new(
            lookat.x() + radius * current_angle.cos(),
            3.7 - (1.5 *(frame as f64 / num_frames as f64)), // Keep camera height constant
            lookat.z() + radius * current_angle.sin() - (0.055* frame as f64),
        );

        let cam = Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );

        // Debug: print image and camera settings for verification
        eprintln!("DEBUG: IMAGE_WIDTH = {}, IMAGE_HEIGHT = {}, lookfrom_y = {}", IMAGE_WIDTH, IMAGE_HEIGHT, lookfrom.y());

        // --- Render a single frame ---
        let filename = format!("output/frame_{:03}.ppm", frame);
        eprintln!("\nRendering frame {}/{} to {}", frame + 1, num_frames, filename);
        let file = File::create(&filename).expect("Failed to create file.");
        let mut writer = BufWriter::new(file);

        writeln!(&mut writer, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).expect("writing header");

        for j in (0..IMAGE_HEIGHT).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, &lights, MAX_DEPTH, custom_bg_color);
                }
                color::write_color(&mut writer, pixel_color, SAMPLES_PER_PIXEL);
            }
        }
    }
 
    eprint!("\nDone.\n");
}