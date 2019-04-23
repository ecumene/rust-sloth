use crate::base::SimpleMesh;
use nalgebra::Matrix4;
use std::f32;

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
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
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
        println!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            x
        );
    }
    pub fn update(&mut self, mut old_size: (u16, u16), meshes: &[SimpleMesh]) {
        let terminal_size = termion::terminal_size().unwrap(); // Temporary size
        if old_size != terminal_size {
            // Check if the size changed
            old_size = terminal_size; // It changed! Set new size
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
            self.width = (old_size.0) as usize;
            self.height = (old_size.1 - 1) as usize;
        }
    }
}
