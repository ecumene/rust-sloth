use crossterm::{cursor, Crossterm, InputEvent, KeyEvent, RawScreen};
use std::error::Error;
use std::f32;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::Instant;
use std::process::Command;

use nalgebra::Rotation3;

pub mod context;
pub use context::*;

pub mod geometry;
pub use geometry::*;

pub mod rasterizer;
pub use rasterizer::*;

pub mod inputs;
pub use inputs::*;

fn main() -> Result<(), Box<Error>> {
    let matches = cli_matches(); // Read command line arguments
    let mut mesh_queue: Vec<SimpleMesh> = vec![]; // A list of meshes to render
    for slice in matches.value_of("INPUT FILENAME").unwrap().split(' ') {
        let error = |s: &str, e: &str| -> Vec<SimpleMesh> {
            println!("filename: [{}] couldn't load, {}. {}", slice, s, e);
            vec![]
        };
        // Fill list with file inputs (Splits for spaces -> multiple files)
        let path = Path::new(slice);
        let mut meshes = match path.extension() {
            None => error("couldn't determine filename extension", ""),
            Some(ext) => match ext.to_str() {
                None => error("couldn't parse filename extension", ""),
                Some(extstr) => match &*extstr.to_lowercase() {
                    "obj" => match tobj::load_obj(&path) {
                        Err(e) => error("tobj couldnt load/parse OBJ", &e.to_string()),
                        Ok(present) => to_meshes(present.0, present.1),
                    },
                    "stl" => match OpenOptions::new().read(true).open(&path) {
                        Err(e) => error("STL load failed", &e.to_string()),
                        Ok(mut file) => match stl_io::read_stl(&mut file) {
                            Err(e) => error("stl_io couldnt parse STL", &e.to_string()),
                            Ok(stlio_mesh) => vec![stlio_mesh.to_simple_mesh()],
                        },
                    },
                    _ => error("unknown filename extension", ""),
                },
            },
        };
        mesh_queue.append(&mut meshes);
    }
    
    let mut turntable = match_turntable(&matches)?;
    let crossterm = Crossterm::new();
    let input = crossterm.input();
    let mut stdin = input.read_async();
    let cursor = cursor();
    let mut no_color = true;
    let mut webify = false;
    let mut webify_frame_count = 0;
    let mut webify_todo_frames = 0;

    let mut context: Context = Context::blank(match_image_mode(&matches)); // The context holds the frame+z buffer, and the width and height
    if context.image {
        if let Some(matches) = matches.subcommand_matches("image") {
            match_dimensions(&mut context, &matches)?;
            turntable = match_turntable(matches)?;
            no_color = match_no_color_mode(matches);
            if let Some(animation_frames) = matches.value_of("frame count") {
                webify_todo_frames = animation_frames.parse()?;
                webify = true;
            }
        }
    } else {
        #[allow(unused)]
        RawScreen::into_raw_mode()?;
        cursor.hide()?;
    }
    let size: (u16, u16) = (0, 0); // This is the terminal size, it's used to check when a new context must be made

    if webify {
        println!("let frames = [");
    }
    let mut last_time; // Used in the variable time step
    loop {
        last_time = Instant::now();
        if let Some(b) = stdin.next() {
            match b {
                InputEvent::Keyboard(event) => match event {
                    KeyEvent::Char('q') => break,
                    _ => {}
                },
                _ => {}
            }
        }
        let rot =
            Rotation3::from_euler_angles(turntable.0, turntable.1, turntable.2).to_homogeneous();
        context.update(size, &mesh_queue)?; // This checks for if there needs to be a context update
        context.clear(); // This clears the z and frame buffer
        for mesh in &mesh_queue {
            // Render all in mesh queue
            draw_mesh(&mut context, &mesh, rot, default_shader); // Draw all meshes
        }

        if webify {
            println!("`");
        }
        
        context.flush(!no_color, webify)?; // This prints all framebuffer info
        stdout().flush().unwrap();
        let dt = Instant::now().duration_since(last_time).as_nanos() as f32 / 1_000_000_000.0;
        turntable.1 += (turntable.3 * dt) as f32; // Turns the turntable

        if webify {
            if turntable.1 > 9.42477 || webify_todo_frames - 1 == webify_frame_count {
                println!("`];");
                break;
            } else {
                println!("`,");
            }
            webify_frame_count+=1;
        }

        if context.image && !webify {
            break;
        }
    }

    cursor.show()?;
    Ok(())
}
