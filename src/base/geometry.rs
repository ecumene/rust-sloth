use nalgebra::{Matrix4, Vector4, Vector2, Unit};
use termion::{color, style};

// 2 3D points = Axis aligned bounding box
pub struct AABB {
    pub min: Vector4<f32>,
    pub max: Vector4<f32>
}

// Functions for the AABB
impl AABB {
    pub fn new(min: Vector4<f32>, max: Vector4<f32>) -> AABB {
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
    pub fn normal(&self) -> Unit<Vector4<f32>> {
        let v1 = self.v2-self.v1;
        let v2 = self.v3-self.v1;
        let x = (v1[1]*v2[2]) - (v1[2]*v2[1]);
        let y = (v1[2]*v2[0]) - (v1[0]*v2[2]);
        let z = (v1[0]*v2[1]) - (v1[1]*v2[0]);
        Unit::new_normalize(Vector4::new(x, y, z, 0.0))
    }
}

trait Material {
    fn shader(&mut self, point: Vector2<f32>);
}

pub struct Mesh {
    pub triangles: Vec<Triangle>
}