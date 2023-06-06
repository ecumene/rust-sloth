mod geom;

use crossterm::style::Color;
use crossterm::style::PrintStyledContent;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::QueueableCommand;
use std::error::Error;

pub use geom::*;
pub use geom::{SimpleMesh, Triangle};
pub use glam::{Mat4, Vec4};

use glam::Vec3;
use std::f32;

use std::io::stdout;
use std::io::Write;

pub mod shader;
pub use shader::*;

pub type Framebuffer = Vec<(char, (u8, u8, u8))>;

pub type Shaderbuffer<const N: usize> = Vec<[Pixel; N]>;

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub z: f32,
    pub shade: f32,
    pub color: Option<(u8, u8, u8)>,
}

impl Pixel {
    pub fn blank() -> Self {
        Self {
            z: f32::MAX,
            shade: 0.0,
            color: None,
        }
    }
}

fn orient(a: &Vec4, b: &Vec4, c: &Vec4) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn orient_triangle(triangle: &Triangle) -> f32 {
    orient(&triangle.v1, &triangle.v2, &triangle.v3)
}

pub struct Rasterizer<'a> {
    pub utransform: Mat4,
    pub meshes: &'a Vec<SimpleMesh>,
}

pub struct Frame<const N: usize> {
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Framebuffer,
    pub shader_buffer: Shaderbuffer<N>,
}

fn flush_charxel(charxel: (char, (u8, u8, u8)), bg_color: Color, stdout: &mut std::io::Stdout) {
    let styled = crossterm::style::style(charxel.0)
        .with(crossterm::style::Color::Rgb {
            r: charxel.1 .0,
            g: charxel.1 .1,
            b: charxel.1 .2,
        })
        .on(bg_color);
    stdout.queue(PrintStyledContent(styled)).unwrap();
}

impl<const N: usize> Frame<N> {
    pub fn blank(width: usize, height: usize) -> Frame<N> {
        Self {
            width,
            height,
            frame_buffer: vec![(' ', (0, 0, 0)); width * height],
            shader_buffer: vec![[Pixel::blank(); N]; width * height as usize],
        }
    }

    pub fn blank_fit_to_terminal() -> Result<Frame<N>, Box<dyn Error>> {
        let size = terminal::size()?;
        let width = size.0 as usize;
        let height = size.1 as usize;
        Ok(Self::blank(width, height))
    }

    pub fn clear(&mut self) {
        self.frame_buffer = vec![(' ', (0, 0, 0)); self.width * self.height];
        self.shader_buffer = vec![[Pixel::blank(); N]; self.width * self.height as usize];
        //f32::MAX is written to the z-buffer as an infinite back-wall to render with
    }

    pub fn flush(&self, bg_color: Color, newlines: bool) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + y * self.width;
                flush_charxel(self.frame_buffer[index], bg_color, &mut stdout)
            }
            if newlines {
                println!();
            }
        }

        stdout.flush()?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn fill(&mut self, charxel: (char, (u8, u8, u8))) {
        for pixel in &mut self.frame_buffer {
            *pixel = charxel;
        }
    }

    pub fn draw_triangle(self: &mut Frame<N>, triangle: &Triangle, transform: Mat4) {
        let mut dist_triangle = triangle.clone();
        dist_triangle.mul(transform);
        let aabb = dist_triangle.aabb(); // Calculate triangle bounds
        let mins: (usize, usize) = (
            aabb.min[0].max(1.0).floor() as usize,
            aabb.min[1].max(1.0).floor() as usize,
        );
        let maxs: (usize, usize) = (
            (aabb.max[0] * 2.0).min((self.width - 1) as f32).ceil() as usize,
            aabb.max[1].min((self.height - 1) as f32).ceil() as usize,
        );
        let a = 1.0 / orient_triangle(&dist_triangle);
        let dim = (N as f32).sqrt();
        for y in mins.1..maxs.1 {
            for x in mins.0..maxs.0 {
                for segment in 0..2 {
                    let spacing: f32 = 0.5 / (dim + 1.0);
                    let id = y * self.width + x * 2;
                    let offset = match segment {
                        0 => -0.5,
                        _ => 0.0,
                    };
                    let mut y_spacing = -0.5 + spacing * 2.0;
                    let mut index = 0;
                    for _ in 0..dim as usize {
                        let mut x_spacing = offset + spacing;
                        for _ in 0..dim as usize {
                            let p = Vec4::new(x as f32 + x_spacing, y as f32 + y_spacing, 0.0, 0.0);
                            let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
                            let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
                            let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
                            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                                let psh = dist_triangle.normal().z * a * (w0 + w1 + w2);
                                let z = dist_triangle.v1[2]
                                    + a * (w1 * (dist_triangle.v2[2] - dist_triangle.v1[2])
                                        + w2 * (dist_triangle.v3[2] - dist_triangle.v1[2]));
                                if z < self.shader_buffer[id + segment][index].z {
                                    self.shader_buffer[id + segment][index] = Pixel {
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

    pub fn draw_mesh(self: &mut Frame<N>, mesh: &SimpleMesh, transform: Mat4) {
        for triangle in &mesh.triangles {
            self.draw_triangle(&triangle, transform);
        }
    }

    pub fn render<S: Shader<N>>(self: &mut Frame<N>, shader: &S) {
        for (i, point) in self.shader_buffer.iter().enumerate() {
            let character = shader.shade_to_char(&point.map(|x| x.shade));
            let dominant_color = match point[N / 2].color {
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
            self.frame_buffer[i] = (character, dominant_color);
        }
    }
}

impl<'a> Rasterizer<'a> {
    pub fn new(meshes: &'a Vec<SimpleMesh>) -> Rasterizer<'a> {
        Rasterizer {
            utransform: Mat4::IDENTITY,
            meshes,
        }
    }

    pub fn scale_to_fit<const N: usize>(
        &mut self,
        frame: &Frame<N>,
        zoom: f32,
    ) -> Result<(), Box<dyn Error>> {
        let mut scale: f32 = 0.0;
        let width = frame.width as f32;
        let height = frame.height as f32;
        for mesh in self.meshes {
            scale = scale
                .max(mesh.bounding_box.max.x)
                .max(mesh.bounding_box.max.y)
                .max(mesh.bounding_box.max.z);
        }

        scale = height.min(width / 2.0) * zoom / scale / 2.0;

        self.utransform = Mat4::from_translation(Vec3::new(width / 4.0, height / 2.0, 0.0))
            * Mat4::from_rotation_y(std::f32::consts::PI)
            * Mat4::from_scale(Vec3::new(scale, -scale, scale));

        Ok(())
    }

    pub fn apply_transform(&mut self, transform: Mat4) {
        self.utransform *= transform;
    }

    pub fn draw_all<const N: usize>(
        self: &Rasterizer<'a>,
        frame: &mut Frame<N>,
        transform: Mat4,
    ) -> Result<(), Box<dyn Error>> {
        frame.clear();

        for mesh in self.meshes {
            frame.draw_mesh(&mesh, self.utransform * transform);
        }

        Ok(())
    }
}

#[cfg(feature = "tui-widget")]
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color as TuiColor, Style},
    widgets::Widget,
};

#[cfg(feature = "tui-widget")]
impl<'a, const N: usize> Widget for Frame<N> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_width = 2;
                for o in 0..pixel_width {
                    let index = x + y * self.width;
                    let charxel = self.frame_buffer[index];
                    let style = Style::default().fg(TuiColor::Rgb(
                        charxel.1 .0,
                        charxel.1 .1,
                        charxel.1 .2,
                    ));
                    if x + o < self.width {
                        buf.get_mut(area.left() + x as u16 + o as u16, area.top() + y as u16)
                            .set_symbol(&charxel.0.to_string())
                            .set_style(style);
                    }
                }
            }
        }
    }
}
