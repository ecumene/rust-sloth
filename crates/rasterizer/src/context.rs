use crate::geom::SimpleMesh;
use glam::{Mat4, Vec3};
use std::error::Error;
use std::f32;

pub type Framebuffer = Vec<(char, (u8, u8, u8))>;

pub struct Context {
    pub utransform: Mat4,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Framebuffer,
    pub z_buffer: Vec<f32>,
}

impl Context {
    pub fn blank() -> Context {
        //TODO: Make this a constant struct
        Context {
            utransform: Mat4::IDENTITY,
            width: 0,
            height: 0,
            frame_buffer: vec![],
            z_buffer: vec![],
        }
    }
    pub fn clear(&mut self) {
        self.frame_buffer = vec![(' ', (0, 0, 0)); self.width * self.height];
        self.z_buffer = vec![f32::MAX; self.width * self.height as usize]; //f32::MAX is written to the z-buffer as an infinite back-wall to render with
    }
    pub fn camera(&mut self, proj: Mat4, view: Mat4) -> &Mat4 {
        self.utransform = proj * view;
        &self.utransform
    }
    pub fn flush(&self, color: bool, webify: bool) -> Result<(), Box<dyn Error>> {
        match (color, webify) {
            (false, _) => {
                let frame: String = self.frame_buffer.iter().map(|pixel| pixel.0).collect();
                println!("{}", frame);
            }
            (true, false) => {
                for pixel in &self.frame_buffer {
                    println!("{:?}", pixel);
                }
            }
            (true, true) => {
                for pixel in &self.frame_buffer {
                    print!(
                        "<span style=\"color:rgb({},{},{})\">{}",
                        (pixel.1).0,
                        (pixel.1).1,
                        (pixel.1).2,
                        pixel.0
                    );
                }
            }
        }

        Ok(())
    }

    pub fn fill(&mut self, charxel: (char, (u8, u8, u8))) {
        for pixel in &mut self.frame_buffer {
            *pixel = charxel;
        }
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
            * Mat4::from_scale(Vec3::new(scale, -scale, scale));

        Ok(())
    }
}
