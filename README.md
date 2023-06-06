# rendersloth - A one-of-a-kind Rust 3D Renderer for the CLI
![pikachu_ascii](models/demo/pikachu_ascii.gif)

![pikachu_unicode](models/demo/pikachu_unicode.gif)
  
Render 3D models in your terminal or app. Sloth is a software rasterizer that
turns triangles into charxels (a character + a colour). It does this via a
simple triangle-grid intersection method to determine if a triangle is in a
character. It then uses a really simple shading scale to determine which
character to use based on brightness. Colour is determined by the Vertex color
for OBJ and the model color for STL.

## Getting Started 
---

### As a library

```
cargo add rendersloth
```

```rust
use rendersloth::*;

// Convert your OBJ to a simpler format for rendering
let mut meshes: Vec<SimpleMesh> = vec![];
let obj_model = tobj::load_obj("file.obj", &tobj::GPU_LOAD_OPTIONS);
let obj_mesh = obj_model.0;
let obj_materials = obj_model.1.expect("Expected to have materials.");
for model in {
    meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
}

let mut context = Rasterizer::new(&meshes);
let mut frame = Frame::blank(50, 20);
let shader = UnicodeShader::new();

// Scale the camera to the model
context.scale_to_fit(&frame, 1.0)?;

// Draw the meshes to the context's built-in shaderbuffer, containing every pixel's shade value
context.draw_all(&mut frame, Mat4::IDENTITY)?;

// Convert the shaderbuffer into a framebuffer, containing ready to be flushed chars with color values
frame.render(&shader);

// Print the screen's contents
frame.flush(crossterm::style::Color::Black, true)?;
```

### Using as a CLI App

```sh
cargo install rendersloth --features build-cli
```

#### Render pikachu
```sh
rendersloth --file-name models/Pikachu.obj --shader unicode turntable
```

Thank you, contributors!
---
[Maxgy](https://github.com/Maxgy) – Rustfmt lint
[donbright](https://github.com/donbright) – STL model loading added, Rustfmt lint
[jonathandturner](https://github.com/jonathandturner) – Crossterm port
