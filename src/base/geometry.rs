use nalgebra::{Matrix4, Vector4, Unit};
use std::clone::Clone;
use tobj::{Mesh};

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
    // This mutates the triangle's points into a given Matrix space
    pub fn mul(&mut self, transform: Matrix4<f32>) -> &mut Triangle {
        self.v1 = &transform*&self.v1;
        self.v2 = &transform*&self.v2;
        self.v3 = &transform*&self.v3;
        self
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

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            v1: self.v1.clone(),
            v2: self.v2.clone(),
            v3: self.v3.clone()
        }
    }
}

pub trait ToSimpleMesh {
    fn to_simple_mesh(&self) -> SimpleMesh;
}

pub struct SimpleMesh {
    pub bounding_box: AABB,
    pub triangles: Vec<Triangle>
}

impl ToSimpleMesh for Mesh {
    fn to_simple_mesh(&self) -> SimpleMesh {
        let mut bounding_box = AABB {
            min: Vector4::new(0.0,0.0,0.0,1.0),
            max: Vector4::new(0.0,0.0,0.0,1.0)
        };
        let mut triangles = vec![Triangle {
            v1: Vector4::new(0.0,0.0,0.0,1.0), 
            v2: Vector4::new(0.0,0.0,0.0,1.0),
            v3: Vector4::new(0.0,0.0,0.0,1.0)
        }; self.indices.len()/3];
        for x in 0..triangles.len() {
            triangles[x].v1.x = self.positions[(self.indices[x*3]*3) as usize];
            triangles[x].v1.y = self.positions[(self.indices[x*3]*3 + 1) as usize];
            triangles[x].v1.z = self.positions[(self.indices[x*3]*3 + 2) as usize];
            triangles[x].v2.x = self.positions[(self.indices[x*3 + 1]*3) as usize];
            triangles[x].v2.y = self.positions[(self.indices[x*3 + 1]*3 + 1) as usize];
            triangles[x].v2.z = self.positions[(self.indices[x*3 + 1]*3 + 2) as usize];
            triangles[x].v3.x = self.positions[(self.indices[x*3 + 2]*3) as usize];
            triangles[x].v3.y = self.positions[(self.indices[x*3 + 2]*3 + 1) as usize];
            triangles[x].v3.z = self.positions[(self.indices[x*3 + 2]*3 + 2) as usize];

            let aabb = triangles[x].to_aabb();
            bounding_box.min.x = aabb.min.x.min(bounding_box.min.x);
            bounding_box.min.y = aabb.min.y.min(bounding_box.min.y);
            bounding_box.min.z = aabb.min.z.min(bounding_box.min.z);
            bounding_box.max.x = aabb.max.x.max(bounding_box.max.x);
            bounding_box.max.y = aabb.max.y.max(bounding_box.max.y);
            bounding_box.max.z = aabb.max.z.max(bounding_box.max.z);
        }
        SimpleMesh { 
            triangles: triangles,
            bounding_box: bounding_box
        }
    }
}