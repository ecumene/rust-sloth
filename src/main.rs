use std::f32;
use std::fs::OpenOptions;
use std::io::{stdout, Read, Write};
use std::path::Path;
use std::time::Instant;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::screen::*;

use nalgebra::{Matrix4, Rotation3};

pub mod base;
pub use base::*;

pub mod inputs;
pub use inputs::*;

fn to_meshes(models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Vec<SimpleMesh> {
    let mut meshes: Vec<SimpleMesh> = vec![];
    for model in models {
        meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
    }
    meshes
}

//TODO: The output blinks very slightly when new output is being posted. Perhaps this is a WSL issue on my part?
fn main() {
    let matches = cli_matches();                    // Read command line arguments
    let mut mesh_queue: Vec<SimpleMesh> = vec![];   // A list of meshes to render
    for slice in matches.value_of("OBJ INPUT").unwrap().split(' ') {
        // Fill list with file inputs (Splits for spaces -> multiple files)
        let unknown = || panic!("unknown file type:{}", slice);
        let path = Path::new(slice);
        let mut meshes = match path.extension() {
            None => unknown(),
            Some(ext) => match ext.to_str().unwrap() {
                "obj" => {
                    let present = tobj::load_obj(&path).unwrap();
                    to_meshes(present.0, present.1)
                }
                "stl" => {
                    let mut file = OpenOptions::new().read(true).open(&path).unwrap();
                    let stl_iomesh = stl_io::read_stl(&mut file).unwrap();
					vec![stl_iomesh.to_simple_mesh()]
                }
                _ => unknown(),
            },
        };
        mesh_queue.append(&mut meshes);
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
        context.update(size, &mesh_queue); // This checks for if there needs to be a context update
        context.clear(); // This clears the z and frame buffer
        for mesh in &mesh_queue {
            // Render all in mesh queue
            draw_mesh(&mut context, &mesh, rot, default_shader); // Draw all meshes
        }
        context.flush(); // This prints all framebuffer info (good for changing colors ;)
        stdout.flush().unwrap();
        let dt = Instant::now().duration_since(last_time).as_nanos() as f32 / 1_000_000_000.0;
        turntable.1 += (speed * dt) as f32; // Turns the turntable
    }
}
