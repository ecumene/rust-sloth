use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    ExecutableCommand,
};
use std::f32;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::{error::Error, io::Stdout};

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
    let fps_cap = 500.0;
    let image_mode = match_image_mode(&matches);
    let mut image_config = None;
    let mut turntable = match_turntable(&matches)?;
    let mut stdout = stdout();
    if image_mode {
        if let Some(matches) = matches.subcommand_matches("image") {
            let mut ic = ImageConfig {
                width: matches.value_of("width").unwrap_or("0").parse()?,
                height: matches.value_of("height").unwrap_or("0").parse()?,
                webify: false,
                webify_todo_frames: 0,
            };
            turntable = match_turntable(matches)?;
            if let Some(animation_frames) = matches.value_of("frame count") {
                ic.webify_todo_frames = animation_frames.parse()?;
                ic.webify = true;
            }
            image_config = Some(ic);
        }
    } else {
        crossterm::terminal::enable_raw_mode()?;
        stdout.execute(cursor::Hide)?;
    }
    let config = SlothConfig {
        fps_cap,
        target_frame_time: Duration::from_secs_f64(1.0 / fps_cap),
        mesh_queue: match_meshes(&matches)?,
        turntable,
        no_color: match_no_color_mode(&matches),
        image_config,
    };
    match matches.value_of("shader") {
        Some("unicode") => run(UnicodeShader::new(), config, &mut stdout),
        _ => run(SimpleShader, config, &mut stdout),
    }
}

fn run<const N: usize, T: Shader<N>>(
    shader: T,
    config: SlothConfig,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    let image_mode = config.image_config.is_some();
    let webify = config
        .image_config
        .as_ref()
        .map(|x| x.webify)
        .unwrap_or(false);
    let mut turntable = config.turntable;
    let mut size: (u16, u16) = (0, 0); // This is the terminal size, it's used to check when a new context must be made

    let mut webify_frame_count = 0;
    if let Some(ic) = &config.image_config {
        size = (ic.width, ic.height);
        if ic.webify {
            println!("let frames = [");
            turntable.3 = (2.0 * f32::consts::PI) * (1.0 / ic.webify_todo_frames as f32);
        }
    }
    let mut context: Context<N> = Context::blank(image_mode); // The context holds the frame+z buffer, and the width and height
    context.set_size(size.0 as usize, size.1 as usize);
    let mut last_time; // Used in the variable time step
    loop {
        last_time = Instant::now();
        if !context.image {
            if poll(config.target_frame_time - last_time.elapsed())? {
                if let Event::Key(KeyEvent { code, modifiers }) = read()? {
                    if code == KeyCode::Char('q')
                        || (code == KeyCode::Char('c') && (modifiers == KeyModifiers::CONTROL))
                    {
                        stdout.execute(cursor::Show)?;
                        crossterm::terminal::disable_raw_mode()?;
                        break;
                    }
                }
            }
        }

        let rot =
            Rotation3::from_euler_angles(turntable.0, turntable.1, turntable.2).to_homogeneous();
        size = context.update(size, &config.mesh_queue)?; // This checks for if there needs to be a context update
        context.clear(); // This clears the z and frame buffer
        for mesh in &config.mesh_queue {
            // Render all in mesh queue
            shader.draw_mesh(&mut context, &mesh, rot); // Draw all meshes
        }
        shader.render(&mut context);

        if webify {
            println!("`");
        }

        context.flush(!config.no_color, webify)?; // This prints all framebuffer info
        stdout.flush()?;
        let dt = Instant::now().duration_since(last_time).as_nanos() as f32 / 1_000_000_000.0;
        turntable.1 += if webify {
            turntable.3
        } else {
            (turntable.3 * dt) as f32
        };

        if let Some(ic) = &config.image_config {
            if ic.webify {
                if turntable.1 > 9.42477 || ic.webify_todo_frames - 1 == webify_frame_count {
                    println!("`];");
                    break;
                } else {
                    println!("`,");
                }
                webify_frame_count += 1;
            }
        }

        if context.image && !webify {
            break;
        }
    }
    Ok(())
}
