use glam::{Mat4, Vec3};
use rasterizer::*;
use tobj::*;

pub fn to_meshes(models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Vec<SimpleMesh> {
    let mut meshes: Vec<SimpleMesh> = vec![];
    for model in models {
        meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
    }
    meshes
}

fn main() {
    let transform = Mat4::IDENTITY;
    let mut context = rasterizer::Context::blank();
    context.width = 40;
    context.height = 40;
    let pika = load_obj("models/cube.obj", &GPU_LOAD_OPTIONS).expect("oops");
    let meshes = to_meshes(pika.0, pika.1.expect("no mats"));

    context.update(&meshes).unwrap();

    rasterizer::draw_all(&mut context, transform, meshes).expect("wow");

    for x in 0..context.width {
        for y in 0..context.height {
            let index = x + y * context.width;
            print!(
                "{}{}",
                context.frame_buffer[index].0, context.frame_buffer[index].0
            );
        }
        println!();
    }
}
