use std::{str, f32};
use nalgebra::{Matrix4};

pub struct Context {
    pub utransform: Matrix4<f32>,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<u8>,
    pub z_buffer: Vec<f32>,
}

impl Context {
    pub fn clear (&mut self) {
        self.frame_buffer = vec![0x0020u8; self.width*self.height as usize];
        self.z_buffer     = vec![f32::MAX; self.width*self.height as usize];
    }
    pub fn camera (&mut self, proj: Matrix4<f32>, view: Matrix4<f32>) -> &Matrix4<f32> {
        self.utransform = proj*view;
        &self.utransform
    }
    pub fn flush (&self) {
        match str::from_utf8(&self.frame_buffer) {
            Ok(v) => println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), v),
            Err(e) => panic!("Invalid UTF-8 shade chosen: {}", e),
        };
    }
}