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
use crate::{light::{compute_light, PointLight}, vec3::Point3};
 
use std::io;
use std::rc::Rc;

use common::random_billiard_color;
use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
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
            let mut color = Color::new(0.0, 0.0, 0.0);
            // Base material scattering
            let mut attenuation = Color::default();
            let mut scattered = Ray::default();
            if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    color += attenuation * ray_color(&scattered, world, lights, depth - 1);
                }
            }

    // Direct lighting from point lights
    for light in lights.iter() {
        // Optional: add shadow check here if needed
      let albedo = attenuation; // or rec.mat’s color if you store it
color += compute_light(rec.p, rec.normal, view_dir, albedo, light);
    }
    return color;
    }
 
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

    let mut color_count = 0;
 
    for a in -2..2 {
        for b in -2..2 {
           // let choose_mat = common::random_double();
            let center = Point3::new(
                a as f64 + 0.8 * common::random_double(), //0.9
                0.2, // * common::random_double(), //original: 0.2,
                b as f64 + 0.8 * common::random_double(), //0.9
            );
 
       //    if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
              //  if choose_mat < 0.4 {
                    // Diffuse
                   // let albedo = Color::random() * Color::random();
                  //  let sphere_material = Rc::new(Lambertian::new(albedo));
                  let albedo = random_billiard_color(color_count);
                  if color_count < 8 {
                  color_count += 1;
                  } else {
                    color_count = 0;
                  }

                  let fuzz = 0.01;
                  let sphere_material = Rc::new(Metal::new(albedo, fuzz));

                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } //else if choose_mat < 0.95 {
                    // Metal
                  //  let albedo = Color::random_range(0.5, 1.0);
                  //  let fuzz = common::random_double_range(0.0, 0.5);
                  //  let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                  //  world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                //} else {
                    // Glass
                //    let sphere_material = Rc::new(Dielectric::new(1.5));
                //    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
               // }
        //    }
    //    }
    }
 
  //  let material1 = Rc::new(Dielectric::new(1.5));
  //  world.add(Box::new(Sphere::new(
  //      Point3::new(0.0, 1.0, 0.0),
  //      1.0,
  //      material1,
  //  )));
 
   // let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
   // world.add(Box::new(Sphere::new(
   //     Point3::new(-4.0, 1.0, 0.0),
   //     1.0,
   //     material2,
   // )));
 
  // let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
  //  world.add(Box::new(Sphere::new(
  //      Point3::new(4.0, 1.0, 0.0),
   //     1.0,
   //     material3,
   // )));
 
    world
}
 
fn main() {
    // Image
 
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1000;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 400;
    const MAX_DEPTH: i32 = 300;
 
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

    let intensity = 0.85;
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