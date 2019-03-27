use nalgebra::{Matrix4, Vector4};
use crate::base::{Triangle, Context, SimpleMesh};

const SHADES: [char; 6] = ['#', '*', '^', '\'', '`', ' '];

pub fn to_char(shade: f32, shades: [char; 6]) -> char {
    if shade <= 0.20 {
        shades[0]
    } else if shade <= 0.40 {
        shades[1]
    } else if shade <= 0.60 {
        shades[2]
    } else if shade <= 0.80 {
        shades[3]
    } else if shade <= 1.0 {
        shades[4]
    } else {
        shades[5]
    }
}

// Used in rasterization
fn orient(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> f32 {
    (b[0]-a[0])*(c[1]-a[1]) - (b[1]-a[1])*(c[0]-a[0])
}

pub fn draw_mesh(context: &mut Context, mesh: &SimpleMesh, transform: Matrix4<f32>){
    for triangle in &mesh.triangles {
        draw_triangle(context, &triangle, transform);
    }
}

pub fn draw_triangle(context: &mut Context, triangle: &Triangle, transform: Matrix4<f32>) {
    let mut dist_triangle = triangle.clone();
    dist_triangle.mul(transform);
    // So we can just render what we need ( within bounds )
    let aabb = dist_triangle.to_aabb();
    let minx = aabb.min[0].max(0.0).floor() as usize;
    let miny = aabb.min[1].max(0.0).floor() as usize;
    let maxx = aabb.max[0].min((context.width-1) as f32).floor() as usize;
    let maxy = aabb.max[1].min((context.height-1) as f32).floor() as usize;
    let a = 1.0 / orient(&dist_triangle.v1, &dist_triangle.v2, &dist_triangle.v3);
    let light = Vector4::new(0.1, -0.283, 0.943, 0.0);
    let shade = dist_triangle.normal().dot(&light) * a;
    for y in miny..maxy { // For Y in bounds
        for x in minx..maxx { // For X in bounds
            let p = Vector4::new(x as f32, y as f32, 0.0, 0.0);
            let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
            let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
            let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
            if w0 >= 0.0 && w1 >= 0.0 && w2 >=0.0 { // Does it past the test?
                let pixel_shade = shade*(w0 + w1 + w2);
                let z = &dist_triangle.v1[2] + a*(w1*(&dist_triangle.v2[2] - &dist_triangle.v1[2]) + w2*(&dist_triangle.v3[2] - &dist_triangle.v1[2]));
                let id = y*context.width + x;
                if z < context.z_buffer[id] {
                    context.z_buffer[id] = z;
                    context.frame_buffer[id] = to_char(pixel_shade, SHADES) as u8; // Sample the bytes -> Sample the shades with 10 thresholds
                }
            }
        }
    }
}