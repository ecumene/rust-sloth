use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use std::io::{Write, stdout, stdin};
use std::{f32};
use std::path::Path;

use nalgebra::{Matrix4, Vector3, Rotation3};

pub mod base;
pub use base::*;

fn to_meshes(models: Vec<tobj::Model>) -> Vec<SimpleMesh> {
    let mut meshes: Vec<SimpleMesh> = vec![];
    for model in models {
        meshes.push(model.mesh.to_simple_mesh());
    }
    meshes
}

fn main() {
    let mut mesh_queue: Vec<SimpleMesh> = vec![];
    &mesh_queue.append(&mut to_meshes(tobj::load_obj(&Path::new("icosphere.obj")).unwrap().0));
    let mut x:f32 = 0.0; 
    let mut y:f32 = 0.0;
    let mut context: Context = Context::blank();
    let mut size: (u16, u16) = (0,0);
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    loop {
        let terminal_size = termion::terminal_size().unwrap(); 
        if size != terminal_size {
            size = terminal_size;
            let scale = size.1 as f32 / 3.0;
            let t = Matrix4::new(scale,   0.0,    0.0, size.0 as f32/4.0, 
                                  0.0,  -scale,   0.0, size.1 as f32/2.0,
                                  0.0,    0.0,  scale,               0.0,
                                  0.0,    0.0,    0.0,               5.0);
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
        context.clear();
        for mesh in &mesh_queue {
            draw_mesh(&mut context, &mesh, &rotx*&roty);
        }
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
