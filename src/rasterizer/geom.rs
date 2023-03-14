#![allow(dead_code)]
use glam::{Mat4, Vec4};
use std::clone::Clone;
use tobj::{Material, Mesh};
use stl_io::IndexedMesh;

#[derive(PartialEq, Debug)]
pub struct AABB {
    pub min: Vec4,
    pub max: Vec4,
}

impl AABB {
    pub fn new(min: Vec4, max: Vec4) -> AABB {
        AABB { min, max }
    }
}

#[derive(PartialEq, Debug)]
pub struct Triangle {
    pub color: (u8, u8, u8),
    pub v1: Vec4,
    pub v2: Vec4,
    pub v3: Vec4,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            color: (1, 0, 0),
            v1: Vec4::new(1.0, -1.0, -1.0, 1.0),
            v2: Vec4::new(-1.0, -1.0, 1.0, 1.0),
            v3: Vec4::new(1.0, 1.0, -1.0, 1.0),
        }
    }
}

impl Triangle {
    pub fn aabb(&self) -> AABB {
        AABB::new(
            Vec4::new(
                self.v1[0].min(self.v2[0].min(self.v3[0])),
                self.v1[1].min(self.v2[1].min(self.v3[1])),
                self.v1[2].min(self.v2[2].min(self.v3[2])),
                1.0,
            ),
            Vec4::new(
                self.v1[0].max(self.v2[0].max(self.v3[0])),
                self.v1[1].max(self.v2[1].max(self.v3[1])),
                self.v1[2].max(self.v2[2].max(self.v3[2])),
                1.0,
            ),
        )
    }
    pub fn mul(&mut self, transform: Mat4) -> &mut Triangle {
        self.v1 = transform * self.v1;
        self.v2 = transform * self.v2;
        self.v3 = transform * self.v3;
        self
    }
    pub fn normal(&self) -> Vec4 {
        let v1 = self.v2 - self.v1;
        let v2 = self.v3 - self.v1;
        let x = (v1[1] * v2[2]) - (v1[2] * v2[1]);
        let y = (v1[2] * v2[0]) - (v1[0] * v2[2]);
        let z = (v1[0] * v2[1]) - (v1[1] * v2[0]);
        Vec4::new(x, y, z, 0.0).normalize()
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            color: self.color,
            v1: self.v1,
            v2: self.v2,
            v3: self.v3,
        }
    }
}

pub trait ToSimpleMesh {
    fn to_simple_mesh(&self) -> SimpleMesh;
}

pub trait ToSimpleMeshWithMaterial {
    fn to_simple_mesh_with_materials(&self, materials: &[Material]) -> SimpleMesh;
}

pub struct SimpleMesh {
    pub bounding_box: AABB,
    pub triangles: Vec<Triangle>,
}

impl ToSimpleMeshWithMaterial for Mesh {
    fn to_simple_mesh_with_materials(&self, materials: &[Material]) -> SimpleMesh {
        let mut bounding_box = AABB {
            min: Vec4::new(0.0, 0.0, 0.0, 1.0),
            max: Vec4::new(0.0, 0.0, 0.0, 1.0),
        };
        let mut triangles = vec![
            Triangle {
                color: (1, 1, 1),
                v1: Vec4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vec4::new(0.0, 0.0, 0.0, 1.0),
                v3: Vec4::new(0.0, 0.0, 0.0, 1.0)
            };
            self.indices.len() / 3
        ];
        for (x, tri) in triangles.iter_mut().enumerate() {
            tri.v1.x = self.positions[(self.indices[x * 3] * 3) as usize];
            tri.v1.y = self.positions[(self.indices[x * 3] * 3 + 1) as usize];
            tri.v1.z = self.positions[(self.indices[x * 3] * 3 + 2) as usize];
            tri.v2.x = self.positions[(self.indices[x * 3 + 1] * 3) as usize];
            tri.v2.y = self.positions[(self.indices[x * 3 + 1] * 3 + 1) as usize];
            tri.v2.z = self.positions[(self.indices[x * 3 + 1] * 3 + 2) as usize];
            tri.v3.x = self.positions[(self.indices[x * 3 + 2] * 3) as usize];
            tri.v3.y = self.positions[(self.indices[x * 3 + 2] * 3 + 1) as usize];
            tri.v3.z = self.positions[(self.indices[x * 3 + 2] * 3 + 2) as usize];

            if !materials.is_empty() {
                let material = &materials[self.material_id.unwrap()];
                tri.color = (
                    (material.diffuse[0] * 255.0) as u8,
                    (material.diffuse[1] * 255.0) as u8,
                    (material.diffuse[2] * 255.0) as u8,
                );

                // Check if the model contains vertex colors.
                if !self.vertex_color.is_empty() {
                    // Get the vertex_color from the first indice in the tri.
                    let color = (
                        (self.vertex_color[(self.indices[x * 3] * 3) as usize] * 255.0) as u8,
                        (self.vertex_color[(self.indices[x * 3] * 3 + 1) as usize] * 255.0) as u8,
                        (self.vertex_color[(self.indices[x * 3] * 3 + 2) as usize] * 255.0) as u8,
                    );
                    tri.color = color;
                }
            }

            let aabb = tri.aabb();
            bounding_box.min.x = aabb.min.x.min(bounding_box.min.x);
            bounding_box.min.y = aabb.min.y.min(bounding_box.min.y);
            bounding_box.min.z = aabb.min.z.min(bounding_box.min.z);
            bounding_box.max.x = aabb.max.x.max(bounding_box.max.x);
            bounding_box.max.y = aabb.max.y.max(bounding_box.max.y);
            bounding_box.max.z = aabb.max.z.max(bounding_box.max.z);
        }
        SimpleMesh {
            triangles,
            bounding_box,
        }
    }
}

impl ToSimpleMesh for Mesh {
    fn to_simple_mesh(&self) -> SimpleMesh {
        self.to_simple_mesh_with_materials(&[])
    }
}

impl ToSimpleMesh for IndexedMesh {
    fn to_simple_mesh(&self) -> SimpleMesh {
        let mut bounding_box = AABB {
            min: Vec4::new(std::f32::MAX, std::f32::MAX, std::f32::MAX, 1.0),
            max: Vec4::new(std::f32::MIN, std::f32::MIN, std::f32::MIN, 1.0),
        };
        fn stlv2v4(stlio_vec: [f32; 3]) -> Vec4 {
            Vec4::new(stlio_vec[0], stlio_vec[1], stlio_vec[2], 1.0)
        }
        let mut triangles = vec![
            Triangle {
                // at time of writing, stl_io lacked color
                color: (0xFF, 0xFF, 0x00),
                v1: Vec4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vec4::new(0.0, 0.0, 0.0, 1.0),
                v3: Vec4::new(0.0, 0.0, 0.0, 1.0)
            };
            self.faces.len()
        ];
        #[allow(clippy::needless_range_loop)]
        // We need an index number, to get the triangle's index too
        for t_index in 0..self.faces.len() {
            triangles[t_index].v1 = stlv2v4(self.vertices[self.faces[t_index].vertices[0]].into());
            triangles[t_index].v2 = stlv2v4(self.vertices[self.faces[t_index].vertices[1]].into());
            triangles[t_index].v3 = stlv2v4(self.vertices[self.faces[t_index].vertices[2]].into());
            let aabb = triangles[t_index].aabb();
            bounding_box.min.x = aabb.min.x.min(bounding_box.min.x);
            bounding_box.min.y = aabb.min.y.min(bounding_box.min.y);
            bounding_box.min.z = aabb.min.z.min(bounding_box.min.z);
            bounding_box.max.x = aabb.max.x.max(bounding_box.max.x);
            bounding_box.max.y = aabb.max.y.max(bounding_box.max.y);
            bounding_box.max.z = aabb.max.z.max(bounding_box.max.z);
        }
        SimpleMesh {
            triangles,
            bounding_box,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Mat4, Vec3, Vec4};

    #[test]
    fn test_aabb() {
        let triangle = Triangle::default();
        assert_eq!(
            triangle.aabb(),
            AABB::new(
                Vec4::new(-1.0, -1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0)
            )
        );
    }

    #[test]
    fn test_transform() {
        let transform = Mat4::from_translation(Vec3::new(1.0, 1.0, 1.0));
        let mut triangle = Triangle::default();
        triangle.mul(transform);
        assert_eq!(
            triangle.aabb(),
            AABB::new(Vec4::new(0.0, 0.0, 0.0, 1.0), Vec4::new(2.0, 2.0, 2.0, 1.0))
        );
    }

    #[test]
    fn test_normal() {
        let triangle = Triangle {
            color: (1, 0, 0),
            v1: Vec4::new(-1.0, 1.0, 0.0, 1.0),
            v2: Vec4::new(0.0, 1.0, 1.0, 1.0),
            v3: Vec4::new(1.0, 1.0, 0.0, 1.0),
        };
        assert_eq!(triangle.normal(), Vec4::new(0.0, 1.0, 0.0, 0.0));
    }
}
