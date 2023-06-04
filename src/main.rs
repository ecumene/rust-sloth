mod rasterizer;

use clap::{Parser, Subcommand, ValueEnum};
use glam::*;
use rasterizer::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::PathBuf;
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
        #[arg(short, long, required = false)]
        speed: f32,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ShaderOptions {
    Simple,
    Unicode,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = false)]
    file_name: PathBuf,

    #[arg(short, long)]
    shader: ShaderOptions,

    #[command(subcommand)]
    mode: Option<Mode>,
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

    match args.shader {
        ShaderOptions::Unicode => Ok(run(UnicodeShader::new(), &meshes)?),
        ShaderOptions::Simple => Ok(run(SimpleShader::new(), &meshes)?),
    }
}

fn run<const N: usize, S: Shader<N>>(
    shader: S,
    meshes: &Vec<SimpleMesh>,
) -> Result<(), Box<dyn Error>> {
    let mut context = Rasterizer::new(&meshes);
    let transform = Mat4::IDENTITY;
    let mut frame = Frame::blank(60, 30);
    context.scale_to_fit(60.0, 30.0);
    context.draw_all(&mut frame, transform)?;
    context.render(&mut frame, shader);
    frame.flush()?;
    Ok(())
}
