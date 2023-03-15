mod geom;

use crossterm::style::PrintStyledContent;
use crossterm::style::Stylize;
use crossterm::QueueableCommand;
use std::error::Error;

pub use geom::*;
pub use geom::{SimpleMesh, Triangle};
pub use glam::{Mat4, Vec4};

use glam::Vec3;
use std::f32;

use std::io::stdout;
use std::io::Write;

#[cfg(feature = "tui-widget")]
use tui::{ widgets::Widget, style::{ Style, Color }, layout::Rect, buffer::Buffer };

pub type Framebuffer = Vec<(char, (u8, u8, u8))>;

fn orient(a: &Vec4, b: &Vec4, c: &Vec4) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn orient_triangle(triangle: &Triangle) -> f32 {
    orient(&triangle.v1, &triangle.v2, &triangle.v3)
}

pub fn default_shader(shade: f32) -> char {
    if shade <= 0.20 {
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

pub struct Rasterizer {
    pub utransform: Mat4,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Framebuffer,
    pub z_buffer: Vec<f32>,
    pub pixel_width: usize,
}

fn flush_charxel(charxel: (char, (u8, u8, u8)), stdout: &mut std::io::Stdout) {
    let styled = crossterm::style::style(charxel.0)
        .with(crossterm::style::Color::Rgb {
            r: charxel.1 .0,
            g: charxel.1 .1,
            b: charxel.1 .2,
        })
        .on(crossterm::style::Color::Black);
    stdout.queue(PrintStyledContent(styled)).unwrap();
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Rasterizer {
        Rasterizer {
            utransform: Mat4::IDENTITY,
            width,
            height,
            frame_buffer: vec![],
            z_buffer: vec![],
            pixel_width: 2,
        }
    }

    pub fn clear(&mut self) {
        self.frame_buffer = vec![(' ', (0, 0, 0)); self.width * self.height];
        self.z_buffer = vec![f32::MAX; self.width * self.height as usize]; //f32::MAX is written to the z-buffer as an infinite back-wall to render with
    }

    pub fn flush(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();
        let pixel_width = 2;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + y * self.width;
                for _ in 0..pixel_width {
                    flush_charxel(self.frame_buffer[index], &mut stdout)
                }
            }
            println!();
        }

        stdout.flush()?;

        Ok(())
    }

    pub fn fill(&mut self, charxel: (char, (u8, u8, u8))) {
        for pixel in &mut self.frame_buffer {
            *pixel = charxel;
        }
    }

    pub fn draw_triangle<F>(self: &mut Rasterizer, triangle: &Triangle, transform: Mat4, shader: F)
    where
        F: Fn(f32) -> char,
    {
        let mut dist_triangle = triangle.clone();
        dist_triangle.mul(self.utransform * transform);
        let aabb = dist_triangle.aabb(); // Calculate triangle bounds
        let mins: (usize, usize) = (
            aabb.min[0].max(1.0).ceil() as usize,
            aabb.min[1].max(1.0).ceil() as usize,
        );
        let maxs: (usize, usize) = (
            (aabb.max[0] * 2.0).min((self.width - 1) as f32).ceil() as usize,
            aabb.max[1].min((self.height - 1) as f32).ceil() as usize,
        );
        let a = 1.0 / orient_triangle(&dist_triangle);

        for y in mins.1..maxs.1 {
            for x in mins.0..maxs.0 {
                let p = Vec4::new(x as f32, y as f32, 0.0, 0.0);
                let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
                let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
                let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let pixel_shade = dist_triangle.normal().z * a * (w0 + w1 + w2);
                    let z = dist_triangle.v1[2]
                        + a * (w1 * (dist_triangle.v2[2] - dist_triangle.v1[2])
                            + w2 * (dist_triangle.v3[2] - dist_triangle.v1[2]));
                    let id = y * self.width + x * 2;
                    if z < self.z_buffer[id] {
                        self.z_buffer[id] = z;
                        let pixel = (shader(pixel_shade), dist_triangle.color);
                        self.frame_buffer[id] = pixel;
                        self.frame_buffer[id + 1] = pixel;
                    }
                }
            }
        }
    }

    pub fn draw_mesh<F>(self: &mut Rasterizer, mesh: &SimpleMesh, transform: Mat4, shader: F)
    where
        F: Fn(f32) -> char,
    {
        for triangle in &mesh.triangles {
            self.draw_triangle(&triangle, transform, &shader);
        }
    }

    pub fn draw_all(
        self: &mut Rasterizer,
        transform: Mat4,
        mesh_queue: Vec<SimpleMesh>,
    ) -> Result<(), Box<dyn Error>> {
        self.update(&mesh_queue)?;
        self.clear();

        for mesh in &mesh_queue {
            self.draw_mesh(&mesh, transform, default_shader);
        }

        Ok(())
    }

    pub fn update(&mut self, meshes: &[SimpleMesh]) -> Result<(), Box<dyn Error>> {
        let mut scale: f32 = 0.0;
        for mesh in meshes {
            scale = scale
                .max(mesh.bounding_box.max.x)
                .max(mesh.bounding_box.max.y)
                .max(mesh.bounding_box.max.z);
        }

        let (width, height) = (self.width as f32, self.height as f32);
        scale = f32::from(height).min(f32::from(width) / 2.0) / scale / 2.0;

        self.utransform = Mat4::from_translation(Vec3::new(width / 4.0, height / 2.0, 0.0))
            * Mat4::from_rotation_y(std::f32::consts::PI)
            * Mat4::from_scale(Vec3::new(scale, -scale * 2.0, scale));

        Ok(())
    }
}

#[cfg(feature = "tui-widget")]
impl Widget for Rasterizer {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + y * self.width;
                let charxel = self.frame_buffer[index];
                let style = Style::default()
                    .fg(Color::Rgb {
                        r: charxel.1 .0,
                        g: charxel.1 .1,
                        b: charxel.1 .2,
                    })
                    .bg(Color::Black);
                buf.get_mut(area.left() + x as u16, area.top() + y as u16)
                    .set_symbol(charxel.0)
                    .set_style(style);
            }
        }
    }
}