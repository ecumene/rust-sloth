# Sloth
![](models/demo/pikachu.gif)
A one-of-a-kind command line 3D software rasterizer made with termion, tobj, and nalgebra. Currently it 
supports OBJ file formats without textures or materials. Here's a really simple command for you to get started.    
`cargo run --release -- models/hand.obj`    
For multiple models:   
`cargo run --release -- "models/suzy.obj models/hand.obj"`   
You can also generate a static image:
`cargo run --release -- image -w <width_in_pixels> -h <height_in_pixels> models/suzy.obj`
You can also generate a portable Javascript render like this:
`cargo run --release -- image -j <number_of_frames> -w <width_in_pixels> -h <height_in_pixels> models/suzy.obj`
