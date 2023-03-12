use glam::Mat4;
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
    context.width = 80;
    context.height = 80;
    let pika = load_obj("models/cube.obj", &GPU_LOAD_OPTIONS).expect("oops");
    let meshes = to_meshes(pika.0, pika.1.expect("no mats"));

    context.update(&meshes).unwrap();
    context.draw_all(transform, meshes).expect("wow");

    context.flush().unwrap();
}
