use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::{f32};
use std::path::Path;
use clap::{App, Arg, ArgMatches};
use std::time::{Duration, Instant};
use std::thread::sleep;

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

fn cli_matches<'a>() -> ArgMatches<'a> {
    App::new("My Super Program")
        .version("0.1")
        .author("Mitchell Hynes. <mshynes@mun.ca>")
        .about("A toy for rendering 3D objects in the command line")
        .arg(Arg::with_name("OBJ INPUT")
            .help("Sets the input obj file to render")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("turntable")
            .short("s")
            .long("turntable")
            .takes_value(true))
            .help("Sets the automatic turntable speed (radians / second in the x direction)")
        .arg(Arg::with_name("rotation")
            .short("r")
            .long("rotation")
            .takes_value(true))
            .help("Sets the object's static rotation (in degrees)")
        .get_matches()
}

fn update_context(mut old_size: (u16, u16), old_context: Context, meshes: &Vec<SimpleMesh>) -> Context {
    let terminal_size = termion::terminal_size().unwrap(); 
    if old_size != terminal_size {
        old_size = terminal_size;
        let mut scale: f32 = 0.0;
        for mesh in meshes {
            scale = scale.max(mesh.bounding_box.max.x).max(mesh.bounding_box.max.y).max(mesh.bounding_box.max.z);
        };
        scale = (old_size.1 as f32).min(old_size.0 as f32 / 2.0) / scale / 2.0;
        let t = Matrix4::new(scale,    0.0,    0.0, old_size.0 as f32/4.0, 
                               0.0, -scale,    0.0, old_size.1 as f32/2.0,
                               0.0,    0.0,  scale,                   0.0,
                               0.0,    0.0,    0.0,                   1.0);
        Context { 
            utransform: t,
            width: (old_size.0) as usize,
            height: (old_size.1 - 1) as usize,
            ..Context::blank()
        }
    } else {
        old_context
    }
}

fn main() {
    let matches = cli_matches();
    let mut mesh_queue: Vec<SimpleMesh> = vec![];
    for slice in matches.value_of("OBJ INPUT").unwrap().split(" "){
        &mesh_queue.append(&mut to_meshes(tobj::load_obj(&Path::new(slice)).unwrap().0));
    }
    let mut speed: f32 = 1.0;
    let mut turntable = (0.0, 0.0, 0.0); 
    if matches.is_present("turntable") {
        speed = matches.value_of("turntable").unwrap().parse().unwrap();
    }
    if matches.is_present("rotation") {
        let value = matches.value_of("rotation").unwrap();
        if value.matches(" ").count() != 2 {
            panic!("Too many arguments in rotation. Format: x y z (in degrees)");
        }
        let rotation: Vec<&str> = value.split(" ").collect();
        turntable = (rotation[0].parse().unwrap(), rotation[1].parse().unwrap(), rotation[2].parse().unwrap());
    }
    let mut context: Context = Context::blank();
    let mut size: (u16, u16) = (0,0);
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut stdin = async_stdin().bytes();
    let mut last_time;
    loop {
        last_time = Instant::now();
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        let rot = Rotation3::from_euler_angles(turntable.0, turntable.1, turntable.2).to_homogeneous();
        context = update_context(size, context, &mesh_queue);
        context.clear();
        for mesh in &mesh_queue {
            draw_mesh(&mut context, &mesh, rot);
        }
        context.flush();
        stdout.flush().unwrap();
        let dt = Instant::now().duration_since(last_time).as_nanos() as f32 / 1000000000.0;
        turntable.1 += (speed * dt) as f32;
    }
    stdout.flush().unwrap();
}
