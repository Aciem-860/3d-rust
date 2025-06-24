//   (2)       (3)
//        *---------*
//       /|        /|
// (0)  / |  (1)  / |
//     *---------*  |
//     |  *------|--*         ^  ^
//     | / (6)   | / (7)    Y | / Z
//     |/        |/           |/
//     *---------*            *----->
// (4)        (5)                 X

use sdl2::pixels::Color;

use crate::{Point3D, Square};

use std::convert::Into;

pub struct Cube {
    vertices: [Point3D; 8],
    color: Color,
}

impl Cube {
    pub fn new(corner: &Point3D, color: Color, edge_size: f32) -> Cube {
        let mut vertices = [Point3D::ZERO; 8];

        let x = &Point3D::X * edge_size;
        let y = &Point3D::Y * edge_size;
        let z = &Point3D::Z * edge_size;

        vertices[0] = corner.clone();
        vertices[1] = corner + &z;
        vertices[2] = corner + &x;
        vertices[3] = corner + &x + z;
        vertices[4] = &vertices[0] - &y;
        vertices[5] = &vertices[1] - &y;
        vertices[6] = &vertices[2] - &y;
        vertices[7] = &vertices[3] - &y;

        Cube { vertices, color }
    }
}

impl Into<Vec<Square>> for Cube {
    fn into(self) -> Vec<Square> {
        let mut faces: Vec<Square> = vec![];

        let top = [self.vertices[0].clone(), self.vertices[1].clone(), self.vertices[3].clone(), self.vertices[2].clone()];
        let top = top.as_slice();

        let bottom = [self.vertices[4].clone(), self.vertices[5].clone(), self.vertices[7].clone(), self.vertices[6].clone()];
        let bottom = bottom.as_slice();

        let front = [self.vertices[0].clone(), self.vertices[1].clone(), self.vertices[5].clone(), self.vertices[4].clone()];
        let front = front.as_slice();

        let left = [self.vertices[0].clone(), self.vertices[2].clone(), self.vertices[6].clone(), self.vertices[4].clone()];
        let left = left.as_slice();

        let right = [self.vertices[1].clone(), self.vertices[3].clone(), self.vertices[7].clone(), self.vertices[5].clone()];
        let right = right.as_slice();

        let back = [self.vertices[2].clone(), self.vertices[3].clone(), self.vertices[7].clone(), self.vertices[6].clone()];
        let back = back.as_slice();


        faces.push(Square::new(top, &self.color));
        faces.push(Square::new(bottom, &self.color));
        faces.push(Square::new(front, &self.color));
        faces.push(Square::new(left, &self.color));
        faces.push(Square::new(right, &self.color));
        faces.push(Square::new(back, &self.color));

        faces
    }
}
