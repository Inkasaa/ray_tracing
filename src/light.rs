use crate::vec3;
use crate::Color;
use crate::vec3::Vec3;
use crate::vec3::Point3;


pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f64,// e.g., white light = Color::new(1.0, 1.0, 1.0)
}

impl PointLight {
    pub fn new(position: Point3, color: Color, intensity: f64) -> Self {
        Self { position, color, intensity }
    }
}
pub fn compute_light(
    hit_point: Point3,
    normal: Vec3,
    view_dir: Vec3,
    albedo: Color,
    light: &PointLight,
) -> Color {
    // Vector from hit point to light
    let to_light = (light.position - hit_point).unit_vector();

    // Diffuse contribution (Lambertian)
    let diff = f64::max(normal.dot(&to_light), 0.0);
    let diffuse = diff * albedo * light.intensity;

    // Specular contribution (Phong-like)
    let reflect_dir = vec3::reflect(-to_light, normal);
    let spec_angle = f64::max(view_dir.dot(&reflect_dir), 0.0);

    let specular_strength = 0.8; // adjust highlight intensity
    let shininess = 50.0;        // controls highlight size
    
    let specular = light.color
    * specular_strength
    * spec_angle.powf(shininess)
    * light.intensity;


    // Total contribution
    diffuse + specular
}

