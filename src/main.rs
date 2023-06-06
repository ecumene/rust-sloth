mod rasterizer;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Color;
use crossterm::{cursor, QueueableCommand};
use glam::*;
use rasterizer::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::stdout;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tobj;

pub fn to_meshes(
    present: (
        Vec<tobj::Model>,
        Result<Vec<tobj::Material>, tobj::LoadError>,
    ),
) -> Vec<SimpleMesh> {
    let models = present.0;
    let materials = present.1.expect("couldn't load materials");
    let mut meshes: Vec<SimpleMesh> = vec![];
    for model in models {
        meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
    }
    meshes
}

#[derive(Subcommand, Debug)]
enum Mode {
    Image {
        #[arg(short, long, required = false)]
        width: usize,

        #[arg(short, long, required = false)]
        height: usize,
    },
    Turntable {
        #[arg(short, long, num_args = 3, default_values = ["0", "50", "0"])]
        rotation: Vec<f32>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ShaderOptions {
    Simple,
    Unicode,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Args {
    #[arg(short, long, required = false)]
    file_name: PathBuf,

    #[arg(short, long)]
    shader: ShaderOptions,

    #[arg(long, num_args = 3, default_values = ["20", "20", "20"])]
    bg_color: Vec<u8>,

    #[arg(long, action = ArgAction::Help, value_parser = clap::value_parser!(bool))]
    help: (),

    #[command(subcommand)]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let error = |s: &str, e: &str| -> Result<Vec<SimpleMesh>, Box<dyn Error>> {
        Err(format!(
            "filename: [{}] couldn't load, {}. {}",
            args.file_name.display(),
            s,
            e
        )
        .into())
    };

    let meshes = match args.file_name.extension() {
        None => error("couldn't determine filename extension", ""),
        Some(ext) => match ext.to_str() {
            None => error("couldn't parse filename extension", ""),
            Some(extstr) => match &*extstr.to_lowercase() {
                "obj" => match tobj::load_obj(&args.file_name, &tobj::GPU_LOAD_OPTIONS) {
                    Err(e) => error("tobj couldnt load/parse OBJ", &e.to_string()),
                    Ok(present) => Ok(to_meshes(present)),
                },
                "stl" => match OpenOptions::new().read(true).open(&args.file_name) {
                    Err(e) => error("STL load failed", &e.to_string()),
                    Ok(mut file) => match stl_io::read_stl(&mut file) {
                        Err(e) => error("stl_io couldnt parse STL", &e.to_string()),
                        Ok(stlio_mesh) => Ok(vec![stlio_mesh.to_simple_mesh()]),
                    },
                },
                _ => error("unknown filename extension", ""),
            },
        },
    }?;

    // TODO: Image + Turntable

    let bg_color = Color::Rgb {
        r: args.bg_color[0],
        g: args.bg_color[1],
        b: args.bg_color[2],
    };

    let mode = args.mode;

    match args.shader {
        ShaderOptions::Unicode => Ok(run(UnicodeShader::new(), &meshes, mode, bg_color)?),
        ShaderOptions::Simple => Ok(run(SimpleShader::new(), &meshes, mode, bg_color)?),
    }
}

fn run<const N: usize, S: Shader<N>>(
    shader: S,
    meshes: &Vec<SimpleMesh>,
    mode: Mode,
    bg_color: Color,
) -> Result<(), Box<dyn Error>> {
    let mut context = Rasterizer::new(&meshes);
    let transform = Mat4::IDENTITY;
    let mut newlines = true;
    let mut stdout = stdout().lock();
    let mut frame = match mode {
        Mode::Image { width, height } => Frame::blank(width, height),
        Mode::Turntable { rotation: _ } => {
            newlines = false;
            crossterm::terminal::enable_raw_mode()?;
            stdout.queue(cursor::Hide)?;
            stdout.queue(cursor::MoveTo(0, 0))?;
            Frame::blank_fit_to_terminal()?
        }
    };
    context.scale_to_fit(&frame)?;
    loop {
        let last_time = Instant::now();
        context.draw_all(&mut frame, transform)?;
        context.render(&mut frame, &shader);
        frame.flush(bg_color, newlines)?;
        match mode {
            Mode::Image {
                width: _,
                height: _,
            } => break,
            Mode::Turntable { ref rotation } => {
                if poll(Duration::from_millis(0))? {
                    if let Event::Key(KeyEvent {
                        code,
                        modifiers,
                        kind: _,
                        state: _,
                    }) = read()?
                    {
                        if code == KeyCode::Char('q')
                            || (code == KeyCode::Char('c') && (modifiers == KeyModifiers::CONTROL))
                        {
                            stdout.queue(cursor::Show)?;
                            crossterm::terminal::disable_raw_mode()?;
                            break;
                        }
                    }
                }
                let dt =
                    Instant::now().duration_since(last_time).as_nanos() as f32 / 1_000_000_000.0;
                let rot = Mat4::from_euler(
                    EulerRot::XYZ,
                    rotation[0] * dt,
                    rotation[1] * dt,
                    rotation[2] * dt,
                );
                context.apply_transform(rot);
                stdout.queue(cursor::MoveTo(0, 0))?;
            }
        }
    }
    Ok(())
}
