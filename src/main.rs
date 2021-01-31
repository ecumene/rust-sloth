use crossterm::{cursor, Crossterm, InputEvent, KeyEvent, RawScreen};
use std::error::Error;
use std::f32;
use std::io::{stdout, Write};
use std::time::Instant;

use nalgebra::Rotation3;

pub mod context;
pub use context::*;

pub mod geometry;
pub use geometry::*;

pub mod rasterizer;
pub use rasterizer::*;

pub mod inputs;
pub use inputs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli_matches(); // Read command line arguments
    let mesh_queue: Vec<SimpleMesh> = match_meshes(&matches)?; // A list of meshes to render
    let mut turntable = match_turntable(&matches)?;
    let crossterm = Crossterm::new();
    let input = crossterm.input();
    let mut stdin = input.read_async();
    let cursor = cursor();
    let no_color = match_no_color_mode(&matches);
    let mut webify = false;
    let mut webify_frame_count = 0;
    let mut webify_todo_frames = 0;

    let mut context: Context = Context::blank(match_image_mode(&matches)); // The context holds the frame+z buffer, and the width and height
    if context.image {
        if let Some(matches) = matches.subcommand_matches("image") {
            match_dimensions(&mut context, &matches)?;
            turntable = match_turntable(matches)?;
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
        turntable.3 = (2.0 * f32::consts::PI) * (1.0 / webify_todo_frames as f32);
    }
    let mut last_time; // Used in the variable time step
    loop {
        last_time = Instant::now();
        if !context.image {
            if let Some(b) = stdin.next() {
                if let InputEvent::Keyboard(event) = b {
                    if let KeyEvent::Char('q') = event {
                        cursor.show()?;
                        break;
                    }
                }
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
        stdout().flush()?;
        let dt = Instant::now().duration_since(last_time).as_nanos() as f32 / 1_000_000_000.0;
        turntable.1 += if webify {
            turntable.3
        } else {
            (turntable.3 * dt) as f32
        };

        if webify {
            if turntable.1 > 9.42477 || webify_todo_frames - 1 == webify_frame_count {
                println!("`];");
                break;
            } else {
                println!("`,");
            }
            webify_frame_count += 1;
        }

        if context.image && !webify {
            break;
        }
    }

    Ok(())
}
