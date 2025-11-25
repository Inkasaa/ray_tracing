mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
mod light;
use crate::{light::{compute_light, PointLight}, material::{Material, Striped}, vec3::Point3};
 
use std::io;
use std::rc::Rc;

use common::*;
use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList; 
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
 
fn ray_color(r: &Ray, world: &dyn Hittable,lights: &[PointLight], depth: i32) -> Color {
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
                    attenuation * ray_color(&scattered, world, lights, depth - 1)
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

    //Background gradient
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
 
fn random_scene() -> HittableList {
    let mut world = HittableList::new();
 
    let ground_material = Rc::new(Lambertian::new(Color::new(0.099, 0.172, 0.095))); //original: 0.5, 0.5, 0.5
    world.add(Box::new(Sphere::new( //huge sphere                                                   //pink: 0.55, 0.252, 0.192
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

 let mut count_balls = 0;
    for a in -1..3 {
        for b in -1..3 {
           // let choose_mat = common::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_double(), //0.9
                0.2,  
                b as f64 + 0.9 * common::random_double(), //0.9
            );

let color = random_billiard_color(count_balls);
   count_balls += 1;

    let fuzz = 0.01;
    let spot_dir = vec3::random_unit_vector();

    let sphere_material: Rc<dyn Material> = if color.is_spots {
        Rc::new(Striped::new(color.color, fuzz, center))
    } else {
        Rc::new(Metal::new(color.color, fuzz, center, spot_dir))
    };

    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));

                } 
    }
 

 
    world
}
 
fn main() {
    // Image
 
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 200;
    const MAX_DEPTH: i32 = 100;
 
    // World
 
    let world = random_scene();
 
    // Camera
 
    let lookfrom = Point3::new(8.0, 3.0, 4.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 20.0;
    let aperture = 0.01;
 
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let intensity = 0.75;
    let lights = vec![
    PointLight::new(Point3::new(0.0, 5.0, 4.0), Color::new(1.0, 0.85, 0.6), intensity), // overhead lamp
];

 
    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, &lights, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
 
    eprint!("\nDone.\n");
}