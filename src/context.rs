use crate::geometry::SimpleMesh;
use crossterm::{cursor, terminal, Attribute, Color, Colored};
use nalgebra::Matrix4;
use std::error::Error;
use std::f32;

pub struct Context {
    pub utransform: Matrix4<f32>,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<(char, (u8, u8, u8))>,
    pub z_buffer: Vec<f32>,
    pub image: bool,
}

impl Context {
    pub fn blank(image: bool) -> Context {
        Context {
            utransform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            width: 0,
            height: 0,
            frame_buffer: vec![],
            z_buffer: vec![],
            image,
        }
    }
    pub fn clear(&mut self) {
        self.frame_buffer = vec![
            (' ', (0, 0, 0));
            if self.image {
                self.width * self.height + self.height
            } else {
                self.width * self.height
            } as usize
        ];
        self.z_buffer = vec![f32::MAX; self.width * self.height as usize]; //f32::MAX is written to the z-buffer as an infinite back-wall to render with
    }
    pub fn camera(&mut self, proj: Matrix4<f32>, view: Matrix4<f32>) -> &Matrix4<f32> {
        self.utransform = proj * view;
        &self.utransform
    }
    pub fn flush(&self, color: bool, webify: bool) -> Result<(), Box<dyn Error>> {
        let mut prev_color = None;

        if !self.image {
            cursor().goto(0, 0)?;
        }

        if color {
            for pixel in &self.frame_buffer {
                match prev_color {
                    Some(c) if c == pixel.1 => {
                        print!("{}", pixel.0);
                    }
                    _ => {
                        prev_color = Some(pixel.1);
                        if webify {
                            print!(
                                "<span style=\"color:rgb({},{},{})\">{}",
                                (pixel.1).0,
                                (pixel.1).1,
                                (pixel.1).2,
                                pixel.0
                            )
                        } else {
                            print!(
                                "{}{}{}",
                                Colored::Fg(Color::Rgb {
                                    r: (pixel.1).0,
                                    g: (pixel.1).1,
                                    b: (pixel.1).2
                                }),
                                Colored::Bg(Color::Rgb {
                                    r: 25,
                                    g: 25,
                                    b: 25
                                }),
                                pixel.0
                            )
                        }
                    }
                }
            }
            if !self.image {
                println!("{}", Attribute::Reset);
            }
        } else {
            let mut frame = String::from("");
            for pixel in &self.frame_buffer {
                frame.push(pixel.0);
            }
            println!("{}", frame)
        }

        Ok(())
    }
    pub fn update(
        &mut self,
        mut old_size: (u16, u16),
        meshes: &[SimpleMesh],
    ) -> Result<(), Box<dyn Error>> {
        let terminal = terminal();
        let terminal_size = if self.image {
            (self.width as u16, self.height as u16)
        } else {
            terminal.terminal_size()
        };

        if old_size != terminal_size {
            old_size = terminal_size; // It changed! Set new size
            old_size.0 += 1;
            let mut scale: f32 = 0.0; // The scene's scale
            for mesh in meshes {
                // This calculates the maximum axis value (x y or z) in all meshes
                scale = scale
                    .max(mesh.bounding_box.max.x)
                    .max(mesh.bounding_box.max.y)
                    .max(mesh.bounding_box.max.z);
            }
            scale = f32::from(old_size.1).min(f32::from(old_size.0) / 2.0) / scale / 2.0; // Constrain to width and height, whichever is smaller
            let t = Matrix4::new(
                scale,
                0.0,
                0.0,
                f32::from(old_size.0) / 4.0, // X translation is divided by 4 because there's a 1 char space between charxels
                0.0,
                -scale,
                0.0,
                f32::from(old_size.1) / 2.0, // Y translation is divided by 2 to center
                0.0,
                0.0,
                scale,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            );
            self.utransform = t;
            if !self.image {
                self.width = old_size.0 as usize;
                self.height = (old_size.1) as usize;
            }
        }

        Ok(())
    }
}
