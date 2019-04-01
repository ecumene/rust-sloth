use nalgebra::Matrix4;
use std::{f32, str};

pub struct Context {
    pub utransform: Matrix4<f32>,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<u8>,
    pub z_buffer: Vec<f32>,
}

impl Context {
    pub fn blank() -> Context {
        //TODO: Make this a constant struct
        Context {
            utransform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            width: 0,
            height: 0,
            frame_buffer: vec![],
            z_buffer: vec![],
        }
    }
    pub fn clear(&mut self) {
        self.frame_buffer = vec![0x0020u8; self.width * self.height as usize]; //0x0020u8 is the default space character
        self.z_buffer = vec![f32::MAX; self.width * self.height as usize]; //f32::MAX is written to the z-buffer as an infinite back-wall to render with
    }
    pub fn camera(&mut self, proj: Matrix4<f32>, view: Matrix4<f32>) -> &Matrix4<f32> {
        self.utransform = proj * view;
        &self.utransform
    }
    pub fn flush(&self) {
        match str::from_utf8(&self.frame_buffer) {
            Ok(v) => println!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                v
            ), // TODO: Create colored materials via 24-bit RGB colors
            Err(e) => panic!("Invalid UTF-8 shade chosen: {}", e),
        };
    }
}
