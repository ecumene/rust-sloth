# sloth - A one-of-a-kind Rust 3D Renderer for the CLI
![pikachu](models/demo/pikachu.gif)
  
A one-of-a-kind command line 3D software rasterizer made with termion, tobj, and nalgebra. Currently it 
supports OBJ file formats without textures.

[Javascript Export Demonstration](http://ecumene.xyz/sloth-demo)

## Getting Started / Uses
---
Here's a few really simple commands for you to get started.

You can replace `sloth <args>` with `cargo run --release -- <args>` anywhere

#### Render pikachu
```
sloth models/Pikachu.obj
```
#### For multiple models:   
```
sloth "models/suzy.obj models/suzy.obj"
```
#### You can also generate a static image:
```
sloth models/Pikachu.obj image -w <width_in_pixels> -h <height_in_pixels>
```
#### You can also generate a portable Javascript render like this:
```
sloth models/Pikachu.obj image -j <number_of_frames> -w <width_in_pixels> -h <height_in_pixels> > src-webify/data.js
```

Thank you, contributors!
---
[Maxgy](https://github.com/Maxgy) – Rustfmt lint
[donbright](https://github.com/donbright) – STL model loading added, Rustfmt lint
[jonathandturner](https://github.com/jonathandturner) – Crossterm port