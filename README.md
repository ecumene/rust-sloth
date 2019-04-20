# Sloth
![](models/demo/pikachu.gif)
A one-of-a-kind command line 3D software rasterizer made with termion, tobj, and nalgebra. Currently it 
supports OBJ file formats without textures or materials. Here's a really simple command for you to get started.    
`cargo build --release`    
`./target/release/sloth -r "0 0 90" hand.obj`    
For multiple models:   
`./target/release/sloth -r "0 0 90" "suzy.obj hand.obj"`   
