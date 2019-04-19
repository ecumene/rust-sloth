use nalgebra::Matrix4;
use std::{f32};

pub struct Context {
    pub utransform: Matrix4<f32>,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<String>,
    pub z_buffer: Vec<f32>,
}

impl Context {
    pub fn blank() -> Context {
        //TODO: Make this a constant struct
        Context {
            utransform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            width: 0,
            height: 0,
            frame_buffer: vec![],
            z_buffer: vec![],
        }
    }
    pub fn clear(&mut self) {
        self.frame_buffer = vec![" ".to_string(); self.width * self.height as usize];
        self.z_buffer = vec![f32::MAX; self.width * self.height as usize]; //f32::MAX is written to the z-buffer as an infinite back-wall to render with
    }
    pub fn camera(&mut self, proj: Matrix4<f32>, view: Matrix4<f32>) -> &Matrix4<f32> {
        self.utransform = proj * view;
        &self.utransform
    }
    pub fn flush(&self) {
        let mut x: String = "".to_string();
        for pixel in &self.frame_buffer {
            x.push_str(pixel);
        }
        println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), x);
    }
}
