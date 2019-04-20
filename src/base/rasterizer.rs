use crate::base::{Context, SimpleMesh, Triangle};
use termion::{color};
use nalgebra::{Matrix4, Vector4};

pub fn default_shader(shade: f32) -> String {
    let string = if shade <= 0.20 {
        "."
    } else if shade <= 0.30 {
        ":"
    } else if shade <= 0.40 {
        "-"
    } else if shade <= 0.50 {
        "="
    } else if shade <= 0.60 {
        "+"
    } else if shade <= 0.70 {
        "*"
    } else if shade <= 0.80 {
        "#"
    } else if shade <= 0.90 {
        "%"
    } else if shade <= 1.0 {
        "@"
    } else {
        " "
    };
    string.to_string()
}

// Used in rasterization
fn orient(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn orient_triangle(triangle: &Triangle) -> f32 {
    orient(&triangle.v1, &triangle.v2, &triangle.v3)
}

// Writes multiple meshes to context
pub fn draw_mesh<F>(context: &mut Context, mesh: &SimpleMesh, transform: Matrix4<f32>, shader: F) where F: Fn(f32) -> String {
    for triangle in &mesh.triangles {
        draw_triangle(context, &triangle, transform, &shader);
    }
}

fn bg(src: String, bg: (u8, u8, u8)) -> String {
    format!("{}{}{}{}", color::Fg(color::Rgb(bg.0, bg.1, bg.2)),
                        color::Bg(color::Rgb(25,25,25)), 
                      src, 
                      color::Fg(color::Reset))
}

pub fn draw_triangle<F>(context: &mut Context, triangle: &Triangle, transform: Matrix4<f32>, shader: F) where F: Fn(f32) -> String {
    let mut dist_triangle = triangle.clone();
    dist_triangle.mul(context.utransform * transform);
    let aabb = dist_triangle.to_aabb(); // Calculate triangle bounds
    let mins: (usize, usize) = (aabb.min[0].max(1.0).ceil() as usize, aabb.min[1].max(1.0).ceil() as usize);
    let maxs: (usize, usize) = ((aabb.max[0] * 2.0).min((context.width - 1) as f32).ceil() as usize, aabb.max[1].min((context.height - 1) as f32).ceil() as usize);
    let a = 1.0 / orient_triangle(&dist_triangle);

    for y in mins.1..maxs.1 {
        for x in mins.0..maxs.0 {
            let p = Vector4::new(x as f32, y as f32, 0.0, 0.0);
            let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
            let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
            let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let pixel_shade = dist_triangle.normal().z * a * (w0 + w1 + w2);
                let z = dist_triangle.v1[2]
                    + a * (w1 * (dist_triangle.v2[2] - dist_triangle.v1[2])
                         + w2 * (dist_triangle.v3[2] - dist_triangle.v1[2]));
                let id = y * context.width + x * 2;
                if z < context.z_buffer[id] {
                    context.z_buffer[id] = z;
                    let pixel = bg(shader(pixel_shade), dist_triangle.color);
                    context.frame_buffer[id] = (&pixel).to_string();
                    context.frame_buffer[id+1] = (&pixel).to_string();
                }
            }
        }
    }
}
