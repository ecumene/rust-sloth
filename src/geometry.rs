use nalgebra::{Matrix4, Unit, Vector4};
use std::clone::Clone;
use tobj::{Material, Mesh};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vector4<f32>,
    pub max: Vector4<f32>,
}

impl AABB {
    pub fn new(min: Vector4<f32>, max: Vector4<f32>) -> AABB {
        AABB { min, max }
    }
}

#[derive(PartialEq, Debug)]
pub struct Triangle {
    pub color: (u8, u8, u8),
    pub v1: Vector4<f32>,
    pub v2: Vector4<f32>,
    pub v3: Vector4<f32>,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            color: (1, 0, 0),
            v1: Vector4::new(1.0, -1.0, -1.0, 1.0),
            v2: Vector4::new(-1.0, -1.0, 1.0, 1.0),
            v3: Vector4::new(1.0, 1.0, -1.0, 1.0),
        }
    }
}

impl Triangle {
    pub fn aabb(&self) -> AABB {
        AABB::new(
            Vector4::from_fn(|x, _size| self.v1[x].min(self.v2[x].min(self.v3[x]))),
            Vector4::from_fn(|x, _size| self.v1[x].max(self.v2[x].max(self.v3[x]))),
        )
    }
    pub fn mul(&mut self, transform: Matrix4<f32>) -> &mut Triangle {
        self.v1 = transform * self.v1;
        self.v2 = transform * self.v2;
        self.v3 = transform * self.v3;
        self
    }
    pub fn normal(&self) -> Unit<Vector4<f32>> {
        let v1 = self.v2 - self.v1;
        let v2 = self.v3 - self.v1;
        let n = v1.xyz().cross(&v2.xyz());
        Unit::new_normalize(n.push(0.0))
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
            // This is the general bounding box for the mesh
            min: Vector4::new(0.0, 0.0, 0.0, 1.0),
            max: Vector4::new(0.0, 0.0, 0.0, 1.0),
        };
        let mut triangles = vec![
            Triangle {
                // Repeat this triangle for all faces in polygon
                color: (1, 1, 1),
                v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v3: Vector4::new(0.0, 0.0, 0.0, 1.0)
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

/// Convert stl_io IndexedMesh into Sloth style triangles.
impl ToSimpleMesh for stl_io::IndexedMesh {
    fn to_simple_mesh(&self) -> SimpleMesh {
        let mut bounding_box = AABB {
            min: Vector4::new(std::f32::MAX, std::f32::MAX, std::f32::MAX, 1.0),
            max: Vector4::new(std::f32::MIN, std::f32::MIN, std::f32::MIN, 1.0),
        };
        fn stlv2v4(stlio_vec: [f32; 3]) -> Vector4<f32> {
            Vector4::new(stlio_vec[0], stlio_vec[1], stlio_vec[2], 1.0)
        }
        let mut triangles = vec![
            Triangle {
                // at time of writing, stl_io lacked color
                color: (0xFF, 0xFF, 0x00),
                v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v2: Vector4::new(0.0, 0.0, 0.0, 1.0),
                v3: Vector4::new(0.0, 0.0, 0.0, 1.0)
            };
            self.faces.len()
        ];
        #[allow(clippy::needless_range_loop)]
        // We need an index number, to get the triangle's index too
        for t_index in 0..self.faces.len() {
            triangles[t_index].v1 = stlv2v4(self.vertices[self.faces[t_index].vertices[0]]);
            triangles[t_index].v2 = stlv2v4(self.vertices[self.faces[t_index].vertices[1]]);
            triangles[t_index].v3 = stlv2v4(self.vertices[self.faces[t_index].vertices[2]]);
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
    use nalgebra::Vector3;

    #[test]
    fn test_aabb() {
        let triangle = Triangle::default();
        assert_eq!(
            triangle.aabb(),
            AABB::new(
                Vector4::new(-1.0, -1.0, -1.0, 1.0),
                Vector4::new(1.0, 1.0, 1.0, 1.0)
            )
        );
    }

    #[test]
    fn test_transform() {
        let transform = Matrix4::new_translation(&Vector3::new(1.0, 1.0, 1.0));
        let mut triangle = Triangle::default();
        triangle.mul(transform);
        assert_eq!(
            triangle.aabb(),
            AABB::new(
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                Vector4::new(2.0, 2.0, 2.0, 1.0)
            )
        );
    }

    #[test]
    fn test_normal() {
        let triangle = Triangle {
            color: (1, 0, 0),
            v1: Vector4::new(-1.0, 1.0, 0.0, 1.0),
            v2: Vector4::new(0.0, 1.0, 1.0, 1.0),
            v3: Vector4::new(1.0, 1.0, 0.0, 1.0),
        };
        assert_eq!(
            triangle.normal(),
            Unit::new_normalize(Vector4::new(0.0, 1.0, 0.0, 0.0))
        );
    }
}
