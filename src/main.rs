
// Simple CPU ray tracer in one file for educational purposes.
// Produces PPM output on stdout. Supports: sphere, plane, cube (AABB), finite cylinder, basic lambertian shading,
// shadows and optional reflections via recursion depth.

use std::env;
use std::f32::{INFINITY, consts::PI};
use std::io::{self, Write};

#[derive(Clone, Copy, Debug)]
struct Vec3 { x: f32, y: f32, z: f32 }

impl Vec3 {
    fn new(x:f32,y:f32,z:f32)->Self{Self{x,y,z}}
    fn dot(self,r:Self)->f32{self.x*r.x + self.y*r.y + self.z*r.z}
    fn length(self)->f32{self.dot(self).sqrt()}
    fn normalized(self)->Self{let l=self.length(); if l==0.0 {self} else {Self::new(self.x/l,self.y/l,self.z/l)}}
    fn sub(self,r:Self)->Self{Self::new(self.x-r.x,self.y-r.y,self.z-r.z)}
    fn add(self,r:Self)->Self{Self::new(self.x+r.x,self.y+r.y,self.z+r.z)}
    fn mul(self,s:f32)->Self{Self::new(self.x*s,self.y*s,self.z*s)}
//   fn component_mul(self,r:Self)->Self{Self::new(self.x*r.x,self.y*r.y,self.z*r.z)}
}

struct Ray { orig: Vec3, dir: Vec3 }

trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

struct Hit { t: f32, point: Vec3, normal: Vec3, color: Vec3, reflective: f32 }

// Sphere
struct Sphere { center: Vec3, radius: f32, color: Vec3, reflective: f32 }
impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.orig.sub(self.center);
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let disc = b*b - 4.0*a*c;
        if disc < 0.0 { return None; }
        let sq = disc.sqrt();
        let mut t = (-b - sq) / (2.0*a);
        if t < 1e-4 {
            t = (-b + sq) / (2.0*a);
            if t < 1e-4 { return None; }
        }
        let p = ray.orig.add(ray.dir.mul(t));
        let n = p.sub(self.center).mul(1.0/self.radius).normalized();
        Some(Hit{t, point: p, normal:n, color:self.color, reflective:self.reflective})
    }
}

// Plane (infinite)
struct Plane { point: Vec3, normal: Vec3, color: Vec3, reflective: f32 }
impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() < 1e-6 { return None; }
        let t = (self.point.sub(ray.orig)).dot(self.normal) / denom;
        if t < 1e-4 { return None; }
        let p = ray.orig.add(ray.dir.mul(t));
        Some(Hit{t, point:p, normal:self.normal.normalized(), color:self.color, reflective:self.reflective})
    }
}

// Axis-aligned box (cube) centered at center, with half-size h
struct AABB { min: Vec3, max: Vec3, color: Vec3, reflective: f32 }
impl Object for AABB {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut tmin = (self.min.x - ray.orig.x) / ray.dir.x;
        let mut tmax = (self.max.x - ray.orig.x) / ray.dir.x;
        if tmin > tmax { std::mem::swap(&mut tmin,&mut tmax); }
        let mut tymin = (self.min.y - ray.orig.y) / ray.dir.y;
        let mut tymax = (self.max.y - ray.orig.y) / ray.dir.y;
        if tymin > tymax { std::mem::swap(&mut tymin,&mut tymax); }
        if (tmin > tymax) || (tymin > tmax) { return None; }
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }
        let mut tzmin = (self.min.z - ray.orig.z) / ray.dir.z;
        let mut tzmax = (self.max.z - ray.orig.z) / ray.dir.z;
        if tzmin > tzmax { std::mem::swap(&mut tzmin,&mut tzmax); }
        if (tmin > tzmax) || (tzmin > tmax) { return None; }
        if tzmin > tmin { tmin = tzmin; }
        // tmin is entry
        let t = if tmin < 1e-4 { tmax } else { tmin };
        if t < 1e-4 { return None; }
        let p = ray.orig.add(ray.dir.mul(t));
        // compute normal by seeing which face is closest
        let mut normal = Vec3::new(0.0,0.0,0.0);
        let eps = 1e-3;
        if (p.x - self.min.x).abs() < eps { normal = Vec3::new(-1.0,0.0,0.0); }
        else if (p.x - self.max.x).abs() < eps { normal = Vec3::new(1.0,0.0,0.0); }
        else if (p.y - self.min.y).abs() < eps { normal = Vec3::new(0.0,-1.0,0.0); }
        else if (p.y - self.max.y).abs() < eps { normal = Vec3::new(0.0,1.0,0.0); }
        else if (p.z - self.min.z).abs() < eps { normal = Vec3::new(0.0,0.0,-1.0); }
        else if (p.z - self.max.z).abs() < eps { normal = Vec3::new(0.0,0.0,1.0); }
        Some(Hit{t, point:p, normal, color:self.color, reflective:self.reflective})
    }
}

// Finite cylinder aligned along Y axis, centered at center, radius r, half-height h
struct Cylinder { center: Vec3, radius: f32, half_h: f32, color: Vec3, reflective: f32 }
impl Object for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // Move to cylinder space
        let ro = ray.orig.sub(self.center);
        let rd = ray.dir;
        // Solve intersection with infinite cylinder x^2+z^2 = r^2
        let a = rd.x*rd.x + rd.z*rd.z;
        let b = 2.0*(ro.x*rd.x + ro.z*rd.z);
        let c = ro.x*ro.x + ro.z*ro.z - self.radius*self.radius;
        let disc = b*b - 4.0*a*c;
        if disc < 0.0 { return None; }
        let sq = disc.sqrt();
        let mut t0 = (-b - sq)/(2.0*a);
        let mut t1 = (-b + sq)/(2.0*a);
        if t0 > t1 { std::mem::swap(&mut t0,&mut t1); }
        // check y bounds for t0 then t1
        for &t in [t0,t1].iter() {
            if t < 1e-4 { continue; }
            let y = ro.y + rd.y * t;
            if y.abs() <= self.half_h {
                let p = ray.orig.add(ray.dir.mul(t));
                let normal = Vec3::new(p.x - self.center.x, 0.0, p.z - self.center.z).normalized();
                return Some(Hit{t, point:p, normal, color:self.color, reflective:self.reflective});
            }
        }
        // check caps (disks)
        // top cap y = +half_h
        if rd.y.abs() > 1e-6 {
            let ttop = (self.center.y + self.half_h - ray.orig.y) / rd.y;
            if ttop > 1e-4 {
                let p = ray.orig.add(ray.dir.mul(ttop));
                let d2 = (p.x - self.center.x)*(p.x - self.center.x) + (p.z - self.center.z)*(p.z - self.center.z);
                if d2 <= self.radius*self.radius { return Some(Hit{t:ttop, point:p, normal:Vec3::new(0.0,1.0,0.0), color:self.color, reflective:self.reflective}); }
            }
            let tbot = (self.center.y - self.half_h - ray.orig.y) / rd.y;
            if tbot > 1e-4 {
                let p = ray.orig.add(ray.dir.mul(tbot));
                let d2 = (p.x - self.center.x)*(p.x - self.center.x) + (p.z - self.center.z)*(p.z - self.center.z);
                if d2 <= self.radius*self.radius { return Some(Hit{t:tbot, point:p, normal:Vec3::new(0.0,-1.0,0.0), color:self.color, reflective:self.reflective}); }
            }
        }
        None
    }
}

// Scene holds heterogenous objects
struct Scene { objects: Vec<Box<dyn Object>> , light_pos: Vec3, ambient: f32 }

impl Scene {
    fn trace(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth == 0 { return Vec3::new(0.0,0.0,0.0); }
        let mut closest_t = INFINITY;
        let mut hit_opt: Option<Hit> = None;
        for obj in &self.objects {
            if let Some(h) = obj.intersect(ray) {
                if h.t < closest_t {
                    closest_t = h.t; hit_opt = Some(h);
                }
            }
        }
        if let Some(hit) = hit_opt {
            // Ambient
            let mut color = hit.color.mul(self.ambient);
            // Diffuse / Lambert
            let to_light = (self.light_pos.sub(hit.point)).normalized();
            // shadow check
            let shadow_ray = Ray{ orig: hit.point.add(hit.normal.mul(1e-4)), dir: to_light };
            let in_light = !self.is_in_shadow(&shadow_ray);
            if in_light {
                let lam = hit.normal.dot(to_light).max(0.0);
                color = color.add(hit.color.mul(lam));
            }
            // Reflection
            if hit.reflective > 0.0 {
                let reflect_dir = reflect(ray.dir.mul(-1.0).normalized(), hit.normal).normalized();
                let refl_ray = Ray{ orig: hit.point.add(hit.normal.mul(1e-4)), dir: reflect_dir };
                let refl_color = self.trace(&refl_ray, depth-1);
                color = color.mul(1.0 - hit.reflective).add(refl_color.mul(hit.reflective));
            }
            return clamp_vec(color);
        }
        // background gradient
        let t = 0.5 * (ray.dir.y + 1.0);
        Vec3::new(1.0,1.0,1.0).mul(1.0 - t).add(Vec3::new(0.5,0.7,1.0).mul(t))
    }

    fn is_in_shadow(&self, ray: &Ray) -> bool {
        for obj in &self.objects {
            if let Some(h) = obj.intersect(ray) {
                // if any hit before the light point, we are in shadow
                // we don't know light distance here; approximate by returning true for any hit
                return true;
            }
        }
        false
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v.sub(n.mul(2.0 * v.dot(n)))
}

fn clamp_vec(mut v: Vec3) -> Vec3 {
    if v.x < 0.0 { v.x = 0.0 } if v.y < 0.0 { v.y = 0.0 } if v.z < 0.0 { v.z = 0.0 }
    if v.x > 1.0 { v.x = 1.0 } if v.y > 1.0 { v.y = 1.0 } if v.z > 1.0 { v.z = 1.0 }
    v
}

// Simple camera that generates rays for an image plane
struct Camera { origin: Vec3, lower_left: Vec3, horizontal: Vec3, vertical: Vec3 }
impl Camera {
    fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov_deg:f32, aspect:f32) -> Self {
        let theta = vfov_deg * PI as f32 / 180.0;
        let half_h = (theta/2.0).tan();
        let half_w = aspect * half_h;
        let w = (lookfrom.sub(lookat)).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);
        let origin = lookfrom;
        let lower_left = origin.sub(u.mul(half_w)).sub(v.mul(half_h)).sub(w);
        let horizontal = u.mul(2.0*half_w);
        let vertical = v.mul(2.0*half_h);
        Camera{origin, lower_left, horizontal, vertical}
    }
    fn get_ray(&self, s:f32, t:f32) -> Ray {
        Ray{ orig: self.origin, dir: (self.lower_left.add(self.horizontal.mul(s)).add(self.vertical.mul(t)).sub(self.origin)).normalized() }
    }
}

// add missing Vec3 methods cross and sub by scalar etc.
impl Vec3 {
    fn cross(self, r: Self) -> Self {
        Self::new(self.y*r.z - self.z*r.y, self.z*r.x - self.x*r.z, self.x*r.y - self.y*r.x)
    }
}

fn color_to_rgb(c: Vec3) -> (u8,u8,u8) {
    let r = (255.99 * c.x) as u8;
    let g = (255.99 * c.y) as u8;
    let b = (255.99 * c.z) as u8;
    (r,g,b)
}

fn main() {
    // parse args minimal: scene (1..4), width height, reflections depth
    let args: Vec<String> = env::args().collect();
    let scene_id = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(3);
    let width = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(400);
    let height = args.get(3).and_then(|s| s.parse::<usize>().ok()).unwrap_or(300);
    let max_depth = args.get(4).and_then(|s| s.parse::<u32>().ok()).unwrap_or(3);

    let aspect = width as f32 / height as f32;

    // Build scene variants
    let mut scene = Scene { objects: vec![], light_pos: Vec3::new(5.0,5.0,-2.0), ambient: 0.1 };

    match scene_id {
        1 => { // single sphere
            scene.objects.push(Box::new(Sphere{center:Vec3::new(0.0,0.0,-3.0), radius:1.0, color:Vec3::new(0.8,0.3,0.3), reflective:0.3}));
            scene.light_pos = Vec3::new(5.0,5.0,-1.0);
        }
        2 => { // plane + cube
            scene.objects.push(Box::new(Plane{point:Vec3::new(0.0,-1.0,0.0), normal:Vec3::new(0.0,1.0,0.0), color:Vec3::new(0.4,0.4,0.4), reflective:0.0}));
            scene.objects.push(Box::new(AABB{min:Vec3::new(-0.5,-1.0,-3.0), max:Vec3::new(0.5,0.0,-2.0), color:Vec3::new(0.2,0.6,0.2), reflective:0.1}));
            scene.light_pos = Vec3::new(2.0,5.0,-1.0);
            scene.ambient = 0.05;
        }
        3 | _ => { // one of each
            scene.objects.push(Box::new(Plane{point:Vec3::new(0.0,-1.0,0.0), normal:Vec3::new(0.0,1.0,0.0), color:Vec3::new(0.6,0.6,0.6), reflective:0.0}));
            scene.objects.push(Box::new(Sphere{center:Vec3::new(-1.2,0.0,-4.0), radius:0.8, color:Vec3::new(0.8,0.2,0.2), reflective:0.3}));
            scene.objects.push(Box::new(AABB{min:Vec3::new(0.6,-1.0,-5.0), max:Vec3::new(1.6,0.0,-4.0), color:Vec3::new(0.2,0.6,0.8), reflective:0.1}));
            scene.objects.push(Box::new(Cylinder{center:Vec3::new(0.0,0.0,-3.5), radius:0.4, half_h:0.7, color:Vec3::new(0.9,0.9,0.2), reflective:0.2}));
            scene.light_pos = Vec3::new(4.0,6.0,-2.0);
            scene.ambient = 0.12;
        }
        4 => { // same as 3 but different camera - handled below
            scene.objects.push(Box::new(Plane{point:Vec3::new(0.0,-1.0,0.0), normal:Vec3::new(0.0,1.0,0.0), color:Vec3::new(0.6,0.6,0.6), reflective:0.0}));
            scene.objects.push(Box::new(Sphere{center:Vec3::new(-1.2,0.0,-4.0), radius:0.8, color:Vec3::new(0.8,0.2,0.2), reflective:0.3}));
            scene.objects.push(Box::new(AABB{min:Vec3::new(0.6,-1.0,-5.0), max:Vec3::new(1.6,0.0,-4.0), color:Vec3::new(0.2,0.6,0.8), reflective:0.1}));
            scene.objects.push(Box::new(Cylinder{center:Vec3::new(0.0,0.0,-3.5), radius:0.4, half_h:0.7, color:Vec3::new(0.9,0.9,0.2), reflective:0.2}));
            scene.light_pos = Vec3::new(4.0,6.0,-2.0);
            scene.ambient = 0.12;
        }
    }

    // Camera
    let (cam_from, cam_at) = if scene_id == 4 {
        (Vec3::new(2.5,1.5,1.0), Vec3::new(0.0,0.0,-3.5))
    } else {
        (Vec3::new(0.0,1.2,1.5), Vec3::new(0.0,0.0,-3.5))
    };
    let cam = Camera::new(cam_from, cam_at, Vec3::new(0.0,1.0,0.0), 50.0, aspect);

    // Render
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "P3").unwrap();
    writeln!(handle, "{} {}", width, height).unwrap();
    writeln!(handle, "255").unwrap();

    for j in 0..height {
        for i in 0..width {
            let u = i as f32 / (width-1) as f32;
            let v = 1.0 - (j as f32 / (height-1) as f32);
            let ray = cam.get_ray(u,v);
            let col = scene.trace(&ray, max_depth);
            let (r,g,b) = color_to_rgb(col);
            writeln!(handle, "{} {} {}", r,g,b).unwrap();
        }
    }
}

