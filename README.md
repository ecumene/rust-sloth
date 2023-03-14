# rendersloth - A one-of-a-kind Rust 3D Renderer for the CLI
![pikachu](models/demo/pikachu.gif)
  
Render 3D models in your terminal or app. Sloth is a software rasterizer that
turns triangles into charxels (a character + a colour). It does this via a
simple triangle-grid intersection method to determine if a triangle is in a
character. It then uses a really simple shading scale to determine which
character to use based on brightness. Colour is determined by the Vertex color
for OBJ and the model color for STL.

## Getting Started 
---

### As a library

```rust
let mut context = Rasterizer::new(40, 40);

// Convert your OBJ to a simpler format for rendering
let mut meshes: Vec<SimpleMesh> = vec![];
let obj_model = tobj::load_obj("file.obj", &tobj::GPU_LOAD_OPTIONS);
let obj_mesh = obj_model.0;
let obj_materials = obj_model.1.expect("Expected to have materials.");
for model in {
    meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
}

// Scale the camera to the model
context.update(&meshes)?;
let transform = Mat4::IDENTITY;
// Draw the meshes to the context's built-in framebuffer
context.draw_all(transform, meshes)?;

// Print the screen's contents
context.flush()?;
```

### Using as a CLI App

```sh
cargo install rendersloth --features build-cli
```

#### Render pikachu
```sh
rendersloth --file-name models/Pikachu.obj
```

Thank you, contributors!
---
[Maxgy](https://github.com/Maxgy) – Rustfmt lint
[donbright](https://github.com/donbright) – STL model loading added, Rustfmt lint
[jonathandturner](https://github.com/jonathandturner) – Crossterm port
