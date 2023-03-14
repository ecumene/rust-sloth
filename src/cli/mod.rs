use tobj;
use clap::{Parser, Subcommand};
use rasterizer::*;

pub fn to_meshes(models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Vec<SimpleMesh> {
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
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = false)]
    file_name: String,

    #[command(subcommand)]
    mode: Mode,
}

fn run() {
    let args = Args::parse();
    println!("{:?}", args);

    let mut context = Rasterizer::new(40, 40);

    let pika = tobj::load_obj("models/ferris.obj", &tobj::GPU_LOAD_OPTIONS).expect("oops");
    let meshes = to_meshes(pika.0, pika.1.expect("no mats"));

    context.update(&meshes).unwrap();
    let transform = Mat4::IDENTITY;
    context.draw_all(transform, meshes).unwrap();

    context.flush().unwrap();
}
