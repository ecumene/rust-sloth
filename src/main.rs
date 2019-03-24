use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use std::io::{Write, stdout, stdin};
use std::{str, f32};

use nalgebra::{Matrix4, Vector4, Vector3, Perspective3, Rotation3};

pub mod base;
pub use base::*;

fn main() {
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mesh = Mesh {
        triangles: vec![
            Triangle {
                v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vector4::new(1.0, 1.0, 1.0, 1.0),
                v3: Vector4::new(-1.0, 1.0, 1.0, 1.0)
            },
            Triangle {
                v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vector4::new(-1.0, 1.0, 1.0, 1.0),
                v3: Vector4::new(-1.0, 1.0, -1.0, 1.0)
            },
            Triangle {
                v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vector4::new(-1.0, 1.0, -1.0, 1.0),
                v3: Vector4::new(1.0, 1.0, -1.0, 1.0)
            },
            Triangle {
                v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vector4::new(1.0, 1.0, -1.0, 1.0),
                v3: Vector4::new(1.0, 1.0, 1.0, 1.0)
            },
        ]
    };
    let persp = Perspective3::new(1.0, 3.14 / 4.0, 1.0, 10.0);
    let persp = persp.as_matrix().clone(); // No more ref!
    let t = Matrix4::new(10.0, 0.0,  0.0,  10.0, 
                         0.0,  10.0, 0.0,  0.0,
                         0.0,  0.0,  10.0, 0.0,
                         0.0,  0.0,  0.0,  10.0);
    let mut x:f32 = 0.0; 
    let mut y:f32 = 0.0;
    let mut context = Context {
        utransform: persp,
        width: 0,
        height: 0,
        frame_buffer: vec![],
        z_buffer: vec![]
    };
    loop {
        let rotx = Rotation3::from_axis_angle(&Vector3::y_axis(), x).to_homogeneous();
        let size = termion::terminal_size().unwrap();
        context.width = (size.0) as usize;
        context.height = (size.1-1) as usize;
        context.clear();
        draw_mesh(&mut context, &mesh, persp*&t*&rotx);
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
