use clap::{App, Arg, ArgMatches};
use std::f32;
use std::io::{stdout, Read, Write};
use std::path::Path;
use std::time::Instant;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::screen::*;

use nalgebra::{Matrix4, Rotation3};

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
        .arg(
            Arg::with_name("OBJ INPUT")
                .help("Sets the input obj file to render")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("turntable")
                .short("s")
                .long("turntable")
                .takes_value(true),
        )
        .help("Sets the automatic turntable speed (radians / second in the x direction)")
        .arg(
            Arg::with_name("rotation")
                .short("r")
                .long("rotation")
                .takes_value(true),
        )
        .help("Sets the object's static rotation (in degrees)")
        .get_matches()
}

fn update_context(
    mut old_size: (u16, u16),
    old_context: Context,
    meshes: &[SimpleMesh],
) -> Context {
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
        let t = Matrix4::new(scale,    0.0,   0.0, f32::from(old_size.0) / 4.0, // X translation is divided by 4 because there's a 1 char space between charxels
                               0.0, -scale,   0.0, f32::from(old_size.1) / 2.0, // Y translation is divided by 2 to center
                               0.0,    0.0, scale,                         0.0,
                               0.0,    0.0,   0.0,                         1.0,
        );
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

//TODO: The output blinks very slightly when new output is being posted. Perhaps this is a WSL issue on my part?
fn main() {
    let matches = cli_matches();                    // Read command line arguments
    let mut mesh_queue: Vec<SimpleMesh> = vec![];   // A list of meshes to render
    for slice in matches.value_of("OBJ INPUT").unwrap().split(' ') {
        // Fill list with file inputs (Splits for spaces -> multiple files)
        mesh_queue.append(&mut to_meshes(tobj::load_obj(&Path::new(slice)).unwrap().0));
    }
    let mut speed: f32 = 1.0;               // Default speed for the x-axis spinning
    let mut turntable = (0.0, 0.0, 0.0);    // Euler rotation variables, quaternions aren't very user friendly
    if matches.is_present("turntable") {
        // Parse turntable speed
        speed = matches.value_of("turntable").unwrap().parse().unwrap();
    }
    if matches.is_present("rotation") {
        let value = matches.value_of("rotation").unwrap(); // Parse initial rotation
        if value.matches(' ').count() != 2 {
            // At least 3 inputs "x y z"
            panic!("Incorrect arguments in rotation. Format: x y z (in degrees)"); // Panic on not enough or too many
        }
        let rotation: Vec<&str> = value.split(' ').collect();
        turntable = (
            rotation[0].parse().unwrap(),
            rotation[1].parse().unwrap(),
            rotation[2].parse().unwrap(),
        );
    }
    let mut context: Context = Context::blank(); // The context holds the frame+z buffer, and the width and height
    let size: (u16, u16) = (0, 0); // This is the terminal size, it's used to check when a new context must be made
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap()); // Raw output is clean output
    let mut stdin = async_stdin().bytes(); // Async in so input isn't blocking
    let mut last_time; // Used in the variable time step
    loop {
        last_time = Instant::now();
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        let rot =
            Rotation3::from_euler_angles(turntable.0, turntable.1, turntable.2).to_homogeneous();
        context = update_context(size, context, &mesh_queue); // This checks for if there needs to be a context update
        context.clear(); // This clears the z and frame buffer
        for mesh in &mesh_queue {
            // Render all in mesh queue
            draw_mesh(&mut context, &mesh, rot); // Draw all meshes
        }
        context.flush(); // This prints all framebuffer info (good for changing colors ;)
        stdout.flush().unwrap();
        let dt = Instant::now().duration_since(last_time).as_nanos() as f32 / 1_000_000_000.0;
        turntable.1 += (speed * dt) as f32; // Turns the turntable
    }
}
