use crate::base::{Context, SimpleMesh, Triangle};
use nalgebra::{Matrix4, Vector4};

const SHADES: [char; 6] = ['#', '*', '^', '\'', '`', ' '];

// Converts shading to
pub fn to_char(shade: f32, shades: [char; 6]) -> char {
    if shade <= 0.20 {
        shades[4]
    } else if shade <= 0.40 {
        shades[3]
    } else if shade <= 0.60 {
        shades[2]
    } else if shade <= 0.80 {
        shades[1]
    } else if shade <= 1.0 {
        shades[0]
    } else {
        shades[5]
    }
}

// Used in rasterization
fn orient(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

// Writes multiple meshes to context
pub fn draw_mesh(context: &mut Context, mesh: &SimpleMesh, transform: Matrix4<f32>) {
    for triangle in &mesh.triangles {
        draw_triangle(context, &triangle, transform);
    }
}

// Writes triangle to context via transform
// TODO: For some reason, the drawing freezes when triangles are out of bounds?
pub fn draw_triangle(context: &mut Context, triangle: &Triangle, transform: Matrix4<f32>) {
    let mut dist_triangle = triangle.clone();
    dist_triangle.mul(transform).mul(context.utransform);
    // So we can just render what we need ( within bounds )
    let aabb = dist_triangle.to_aabb(); // Calculate triangle bounds
    let minx = aabb.min[0].max(1.0).ceil() as usize; // Termion is 1 based
    let miny = aabb.min[1].max(1.0).ceil() as usize; // We use ceiling here to ensure that there's no cap between polys
    let maxx = (aabb.max[0] * 2.0).min((context.width - 1) as f32).ceil() as usize;
    let maxy = aabb.max[1].min((context.height - 1) as f32).ceil() as usize;
    let a = 1.0 / orient(&dist_triangle.v1, &dist_triangle.v2, &dist_triangle.v3);
    let shade = dist_triangle.normal().z * a; // Using the Z component of the normal is the same as a front-facing light
    
    for y in miny..maxy {
        // For Y in bounds
        for x in minx..maxx {
            // For X in bounds
            let p = Vector4::new(x as f32, y as f32, 0.0, 0.0);
            let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
            let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
            let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                // Backface culling isn't necessary because it does not render the backfaces anyway
                let pixel_shade = shade * (w0 + w1 + w2); // This is the lighting component
                let z = dist_triangle.v1[2]
                    + a * (w1 * (dist_triangle.v2[2] - dist_triangle.v1[2])
                        + w2 * (dist_triangle.v3[2] - dist_triangle.v1[2])); // Z-test
                let id = y * context.width + x * 2;
                if z < context.z_buffer[id] {
                    context.z_buffer[id] = z;
                    context.frame_buffer[id] = to_char(pixel_shade, SHADES) as u8; // Sample the bytes -> Sample the shades with 10 thresholds
                }
            }
        }
    }
}
