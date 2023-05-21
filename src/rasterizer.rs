use crate::context::Context;
use crate::geometry::{SimpleMesh, Triangle};
use crate::{Pixel, Shader};
use nalgebra::{Matrix4, Vector4};

// Used in rasterization
fn orient(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn orient_triangle(triangle: &Triangle) -> f32 {
    orient(&triangle.v1, &triangle.v2, &triangle.v3)
}

// Writes multiple meshes to context
pub fn draw_mesh(
    context: &mut Context,
    mesh: &SimpleMesh,
    transform: Matrix4<f32>,
    shader: &Shader,
) {
    for triangle in &mesh.triangles {
        draw_triangle(context, &triangle, transform, &shader);
    }
}

pub fn render(context: &mut Context, shader: &Shader) {
    for (i, point) in context.shader_buffer.iter().enumerate() {
        if point.len() == 0 {
            continue;
        }
        let character =
            shader.shader_to_char(&point.iter().map(|x| x.shade).collect::<Vec<f32>>()[..]);
        let shp = shader.precision();
        let dominant_color = match point[shp / 2].color {
            Some(c) => c,
            None => point
                .iter()
                .max_by(|a, b| {
                    a.shade
                        .partial_cmp(&b.shade)
                        .unwrap_or(std::cmp::Ordering::Less)
                })
                .unwrap()
                .color
                .unwrap(),
        };
        context.frame_buffer[i] = (character, dominant_color);
    }
}

pub fn draw_triangle(
    context: &mut Context,
    triangle: &Triangle,
    transform: Matrix4<f32>,
    shader: &Shader,
) {
    let mut dist_triangle = triangle.clone();
    dist_triangle.mul(context.utransform * transform);
    let aabb = dist_triangle.aabb(); // Calculate triangle bounds
    let mins: (usize, usize) = (
        aabb.min[0].max(1.0).ceil() as usize,
        aabb.min[1].max(1.0).ceil() as usize,
    );
    let maxs: (usize, usize) = (
        (aabb.max[0] * 2.0).min((context.width - 1) as f32).ceil() as usize,
        aabb.max[1].min((context.height - 1) as f32).ceil() as usize,
    );
    let a = 1.0 / orient_triangle(&dist_triangle);

    for y in mins.1..maxs.1 {
        for x in mins.0..maxs.0 {
            for segment in 0..2 {
                let shp = shader.precision();
                let mut points = vec![vec![0.0; shp]; shp];
                let spacing: f32 = 0.5 / (shp as f32 + 1.0);
                let id = y * context.width + x * 2;
                let offset = match segment {
                    0 => -0.5,
                    _ => 0.0,
                };
                let mut y_spacing = -0.5 + spacing * 2.0;
                let mut index = 0;
                for point_y in points.iter_mut() {
                    let mut x_spacing = offset + spacing;
                    for point_x in point_y.iter_mut() {
                        let p = Vector4::new(x as f32 + x_spacing, y as f32 + y_spacing, 0.0, 0.0);
                        let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
                        let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
                        let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
                        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                            let psh = dist_triangle.normal().z * a * (w0 + w1 + w2);
                            let z = dist_triangle.v1[2]
                                + a * (w1 * (dist_triangle.v2[2] - dist_triangle.v1[2])
                                    + w2 * (dist_triangle.v3[2] - dist_triangle.v1[2]));
                            if context.shader_buffer[id + segment].len() == 0 {
                                context.shader_buffer[id + segment] = vec![
                                    Pixel {
                                        shade: 0.0,
                                        z: f32::MAX,
                                        color: None
                                    };
                                    shp * shp
                                ]
                            }
                            if z < context.shader_buffer[id + segment][index].z {
                                context.shader_buffer[id + segment][index] = Pixel {
                                    shade: psh,
                                    z,
                                    color: Some(dist_triangle.color),
                                };
                            }
                        }
                        x_spacing += spacing;
                        index += 1;
                    }
                    y_spacing += spacing * 2.0;
                }
            }
        }
        if context.image {
            context.frame_buffer[y * context.width + 1] = ('\n', (0, 0, 0));
        }
    }
}
