use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use std::io::{Write, stdout, stdin};
use std::{str, f32};
use std::path::Path;

use nalgebra::{Matrix4, Vector4, Vector3, Perspective3, Rotation3};

pub mod base;
pub use base::*;

fn main() {
    let m = tobj::load_obj(&Path::new("icosphere.obj"));
    let mesh = m.unwrap().0[0].mesh.to_simple_mesh();
    let mut x:f32 = 0.0; 
    let mut y:f32 = 0.0;
    let mut context: Context = Context::blank();
    let mut size: (u16, u16) = (0,0);
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    loop {
        if size != termion::terminal_size().unwrap() {
            size = termion::terminal_size().unwrap();
            let scale = size.1 as f32 / 2.0;
            let t = Matrix4::new(scale,   0.0,   0.0,  size.0 as f32/4.0, 
                                  0.0,  scale,   0.0,  size.1 as f32/2.0,
                                  0.0,   0.0,  scale,   0.0,
                                  0.0,   0.0,   0.0,  5.0);
            context = Context { 
                utransform: t,
                width: (size.0) as usize,
                height: (size.1 - 1) as usize,
                frame_buffer: vec![],
                z_buffer: vec![]
            };
        }
        let rotx = Rotation3::from_axis_angle(&Vector3::y_axis(), x).to_homogeneous();
        let roty = Rotation3::from_axis_angle(&Vector3::z_axis(), y).to_homogeneous();
        let size = termion::terminal_size().unwrap();
        context.clear();
        draw_mesh(&mut context, &mesh, &rotx*&roty);
        context.flush();
        stdout.flush().unwrap();

        let c = stdin().keys().next().unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Right => x += 0.05,
            Key::Left => x -= 0.05,
            Key::Up => y += 0.05,
            Key::Down => y -= 0.05,
            _=> {
            }
        }
    }
    stdout.flush().unwrap();
}
