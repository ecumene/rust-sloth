use crate::context::Context;
use crate::geometry::{SimpleMesh, Triangle};
use crate::Pixel;
use kd_tree::{KdPoint, KdTree, KdTreeN};
use nalgebra::{Matrix4, Vector4};
use typenum::U9;

// Used in rasterization
fn orient(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn orient_triangle(triangle: &Triangle) -> f32 {
    orient(&triangle.v1, &triangle.v2, &triangle.v3)
}

pub trait Shader<const SIZE: usize> {
    fn shade_to_char(&self, shade: &[f32; SIZE]) -> char;
    // Writes multiple meshes to context
    fn draw_mesh(&self, context: &mut Context<SIZE>, mesh: &SimpleMesh, transform: Matrix4<f32>) {
        for triangle in &mesh.triangles {
            self.draw_triangle(context, &triangle, transform);
        }
    }

    fn render(&self, context: &mut Context<SIZE>) {
        let mut newline_offset = 0;
        for (i, point) in context.shader_buffer.iter().enumerate() {
            let character = self.shade_to_char(&point.map(|x| x.shade));
            let dominant_color = match point[SIZE / 2].color {
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
                    .unwrap_or((0, 0, 0)),
            };
            context.frame_buffer[i + newline_offset] = (character, dominant_color);
            if context.image && i != 0 && i % context.width == 0 {
                newline_offset += 1;
                context.frame_buffer[i + newline_offset] = ('\n', (0, 0, 0));
            }
        }
    }

    fn draw_triangle(
        &self,
        context: &mut Context<SIZE>,
        triangle: &Triangle,
        transform: Matrix4<f32>,
    ) {
        let mut dist_triangle = triangle.clone();
        dist_triangle.mul(context.utransform * transform);
        let aabb = dist_triangle.aabb(); // Calculate triangle bounds
        let mins: (usize, usize) = (
            aabb.min[0].max(1.0).floor() as usize,
            aabb.min[1].max(1.0).floor() as usize,
        );
        let maxs: (usize, usize) = (
            (aabb.max[0] * 2.0).min((context.width - 1) as f32).ceil() as usize,
            aabb.max[1].min((context.height - 1) as f32).ceil() as usize,
        );
        let a = 1.0 / orient_triangle(&dist_triangle);
        let dim = (SIZE as f32).sqrt();
        for y in mins.1..maxs.1 {
            for x in mins.0..maxs.0 {
                for segment in 0..2 {
                    let spacing: f32 = 0.5 / (dim + 1.0);
                    let id = y * context.width + x * 2;
                    let offset = match segment {
                        0 => -0.5,
                        _ => 0.0,
                    };
                    let mut y_spacing = -0.5 + spacing * 2.0;
                    let mut index = 0;
                    for _ in 0..dim as usize {
                        let mut x_spacing = offset + spacing;
                        for _ in 0..dim as usize {
                            let p =
                                Vector4::new(x as f32 + x_spacing, y as f32 + y_spacing, 0.0, 0.0);
                            let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
                            let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
                            let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
                            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                                let psh = dist_triangle.normal().z * a * (w0 + w1 + w2);
                                let z = dist_triangle.v1[2]
                                    + a * (w1 * (dist_triangle.v2[2] - dist_triangle.v1[2])
                                        + w2 * (dist_triangle.v3[2] - dist_triangle.v1[2]));
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
        }
    }
}

pub struct SimpleShader;

impl SimpleShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Shader<1> for SimpleShader {
    fn shade_to_char(&self, shade: &[f32; 1]) -> char {
        let shade = shade[0];
        if shade <= 0.05 {
            ' '
        } else if shade <= 0.20 {
            '.'
        } else if shade <= 0.30 {
            ':'
        } else if shade <= 0.40 {
            '-'
        } else if shade <= 0.50 {
            '='
        } else if shade <= 0.60 {
            '+'
        } else if shade <= 0.70 {
            '*'
        } else if shade <= 0.80 {
            '#'
        } else if shade <= 0.90 {
            '%'
        } else if shade <= 1.0 {
            '@'
        } else {
            ' '
        }
    }
}

pub struct UnicodeShader(KdTreeN<Shade, U9>);

impl UnicodeShader {
    pub fn new() -> Self {
        let unicode_chars: KdTreeN<Shade, U9> = KdTree::build_by_ordered_float(vec![
            // full blocks
            Shade {
                ch: ' ',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '░',
                p: [0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2],
            },
            Shade {
                ch: '▒',
                p: [0.45, 0.45, 0.45, 0.45, 0.45, 0.45, 0.45, 0.45, 0.45],
            },
            Shade {
                ch: '🮐',
                p: [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            },
            Shade {
                ch: '▓',
                p: [0.85, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85],
            },
            Shade {
                ch: '█',
                p: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            },
            // triangle shades
            Shade {
                ch: '🮜',
                p: [0.5, 0.5, 0.25, 0.5, 0.25, 0.0, 0.25, 0.0, 0.0],
            },
            Shade {
                ch: '🮝',
                p: [0.25, 0.5, 0.5, 0.0, 0.25, 0.5, 0.0, 0.0, 0.25],
            },
            Shade {
                ch: '🮞',
                p: [0.0, 0.0, 0.25, 0.0, 0.25, 0.5, 0.25, 0.5, 0.5],
            },
            Shade {
                ch: '🮟',
                p: [0.25, 0.0, 0.0, 0.5, 0.25, 0.0, 0.5, 0.5, 0.25],
            },
            // shaded halves
            Shade {
                ch: '🮏',
                p: [0.0, 0.0, 0.0, 0.2, 0.2, 0.2, 0.4, 0.4, 0.4],
            },
            Shade {
                ch: '🮎',
                p: [0.4, 0.4, 0.4, 0.2, 0.2, 0.2, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🮌',
                p: [0.5, 0.25, 0.0, 0.5, 0.25, 0.0, 0.5, 0.25, 0.0],
            },
            Shade {
                ch: '🮍',
                p: [0.0, 0.25, 0.5, 0.0, 0.25, 0.5, 0.0, 0.25, 0.25],
            },
            Shade {
                ch: '🮑',
                p: [1.0, 1.0, 1.0, 0.8, 0.8, 0.8, 0.6, 0.6, 0.6],
            },
            Shade {
                ch: '🮒',
                p: [0.6, 0.6, 0.6, 0.8, 0.8, 0.8, 1.0, 1.0, 1.0],
            },
            Shade {
                ch: '▌',
                p: [1.0, 0.5, 0.0, 1.0, 0.5, 0.0, 1.0, 0.5, 0.0],
            },
            Shade {
                ch: '▐',
                p: [0.0, 0.5, 1.0, 0.0, 0.5, 1.0, 0.0, 0.5, 1.0],
            },
            // full corner triangles
            Shade {
                ch: '🭗',
                p: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🭢',
                p: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🭇',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            },
            Shade {
                ch: '🬼',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            },
            // side triangles
            Shade {
                ch: '🢑',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            },
            Shade {
                ch: '🢓',
                p: [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🞀',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🢒',
                p: [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            // big side triangles
            Shade {
                ch: '🭯',
                p: [0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 1.0, 0.5],
            },
            Shade {
                ch: '🭭',
                p: [0.5, 1.0, 0.5, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🭮',
                p: [0.0, 0.0, 0.5, 0.0, 0.5, 1.0, 0.0, 0.0, 0.5],
            },
            Shade {
                ch: '🭬',
                p: [0.5, 0.0, 0.0, 1.0, 0.5, 0.0, 0.5, 0.0, 0.0],
            },
            // other
            Shade {
                ch: '▂',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            },
            Shade {
                ch: '🬰',
                p: [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            },
            Shade {
                ch: '🮔',
                p: [0.6, 0.6, 0.6, 0.8, 0.8, 0.8, 1.0, 1.0, 1.0],
            },
        ]);
        Self(unicode_chars)
    }
}

impl Shader<9> for UnicodeShader {
    fn shade_to_char(&self, shade: &[f32; 9]) -> char {
        self.0.nearest(shade).unwrap().item.ch
    }
}

pub struct Shade {
    p: [f32; 9],
    ch: char,
}

impl KdPoint for Shade {
    type Scalar = f32;
    type Dim = typenum::U9;
    fn at(&self, k: usize) -> f32 {
        self.p[k]
    }
}
