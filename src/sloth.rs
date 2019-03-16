use nalgebra::{Matrix4, Vector4};

const SHADES: [char; 6] = ['#', '*', '^', '\'', '`', ' '];

// 2 3D points = Axis aligned bounding box
pub struct AABB {
    min: Vector4<f32>,
    max: Vector4<f32>
}

// Functions for the AABB
impl AABB {
    fn new(min: Vector4<f32>, max: Vector4<f32>) -> AABB {
        AABB {
            min: min,
            max: max
        }
    }
}

// Three Points in 3D = Triangle
pub struct Triangle {
    pub v1: Vector4<f32>,
    pub v2: Vector4<f32>,
    pub v3: Vector4<f32>
}

// Functions for Triangle Struct
impl Triangle {
    pub fn to_aabb(&self) -> AABB {
        // Forgive me for this brag, but this is the best thing I've written in rust
        AABB::new(
            Vector4::from_fn(|x, _size| self.v1[x].min(self.v2[x].min(self.v3[x]))),
            Vector4::from_fn(|x, _size| self.v1[x].max(self.v2[x].max(self.v3[x])))
        )   
    }
    // This clones the triangle (Its points)
    pub fn clone(&self) -> Triangle {
        Triangle {
            v1: self.v1.clone(),
            v2: self.v2.clone(),
            v3: self.v3.clone()
        }
    }
    // This mutates the triangle's points into a given Matrix space
    pub fn mul(&mut self, transform: Matrix4<f32>) {
        self.v1 = &transform*&self.v1;
        self.v2 = &transform*&self.v2;
        self.v3 = &transform*&self.v3;
    }
}

pub fn to_char(shade: f32, shades: [char; 6]) -> char {
    if shade <= 0.20 {
        shades[0]
    } else if shade <= 0.40 {
        shades[1]
    } else if shade <= 0.60 {
        shades[2]
    } else if shade <= 0.80 {
        shades[3]
    } else if shade <= 1.0 {
        shades[4]
    } else {
        shades[5]
    }
}

// Used in rasterization
fn orient(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> f32 {
    (b[0]-a[0])*(c[1]-a[1]) - (b[1]-a[1])*(c[0]-a[0])
}

pub fn draw_triangle(frame_buffer: &mut Vec<u8>, z_buffer: &mut Vec<f32>, triangle: &Triangle, transform: Matrix4<f32>, width: usize, height: usize) {
    let mut dist_triangle = triangle.clone();
    dist_triangle.mul(transform);
    // So we can just render what we need ( within bounds )
    let aabb = dist_triangle.to_aabb();
    let minx = aabb.min[0].max(0.0).floor() as usize;
    let miny = aabb.min[1].max(0.0).floor() as usize;
    let maxx = aabb.max[0].min((width-1) as f32).floor() as usize;
    let maxy = aabb.max[1].min((height-1) as f32).floor() as usize;
    let a = 1.0 / orient(&dist_triangle.v1, &dist_triangle.v2, &dist_triangle.v3);
    for y in miny..maxy { // For Y in bounds
        for x in minx..maxx { // For X in bounds
            let p = Vector4::new(x as f32, y as f32, 0.0, 0.0);
            let w0 = orient(&dist_triangle.v2, &dist_triangle.v3, &p);
            let w1 = orient(&dist_triangle.v3, &dist_triangle.v1, &p);
            let w2 = orient(&dist_triangle.v1, &dist_triangle.v2, &p);
            if w0>=0.0 && w1>=0.0 && w2 >=0.0 { // Does it past the test?
                let z = &dist_triangle.v1[2] + a*(w1*(&dist_triangle.v2[2] - &dist_triangle.v1[2]) + w2*(&dist_triangle.v3[2] - &dist_triangle.v1[2]));
                let shade = z/10.0;
                let id = y*width + x;
                if z < z_buffer[id] {
                    z_buffer[id] = z;
                    frame_buffer[id] = to_char(shade, SHADES) as u8; // Sample the bytes -> Sample the shades with 10 thresholds
                }
            }
        }
    }
}