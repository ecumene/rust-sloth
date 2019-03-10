// 2 3D points = Axis aligned bounding box
pub struct AABB {
    min: Vector<f32>,
    max: Vector<f32>
}

// Functions for the AABB
impl AABB {
    fn new(min: Vector<f32>, max: Vector<f32>) -> AABB {
        AABB {
            min: min,
            max: max
        }
    }
}

// Three Points in 3D = Triangle
pub struct Triangle {
    pub v1: Vector<f32>,
    pub v2: Vector<f32>,
    pub v3: Vector<f32>
}

// Functions for Triangle Struct
impl Triangle {
    pub fn to_aabb(&self) -> AABB {
        // Forgive me for this brag, but this is the best thing I've written in rust
        AABB::new(
            Vector::from_fn(3, |x| self.v1[x].min(self.v2[x].min(self.v3[x]))),
            Vector::from_fn(3, |x| self.v1[x].max(self.v2[x].max(self.v3[x])))
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
    // This mutates the triangle's points into a given matrix space
    pub fn mul(&mut self, transform: Matrix<f32>) {
        self.v1 = &transform*&self.v1;
        self.v2 = &transform*&self.v2;
        self.v3 = &transform*&self.v3;
    }
}