use sdl2::pixels::Color;

use crate::{Point3D, Tuple};

#[derive(Debug)]
pub struct Square {
    pub vertices: [Point3D; 4],
    pub color: Color,
}

pub struct SquareIter<'a> {
    square: &'a Square,
    idx: usize,
}

impl Square {
    pub fn new(vertices: &[Point3D], color: &Color) -> Square {
        let mut v: [Point3D; 4] = [Point3D::ZERO; 4];
        v.clone_from_slice(vertices);
        Square {
            vertices: v,
            color: *color,
        }
    }

    pub fn iter_pairs(&self) -> SquareIter<'_> {
        SquareIter {
            square: self,
            idx: 0,
        }
    }

    // Return is a vector whose origin is at (0, 0, 0)
    pub fn normal(&self) -> Point3D {
        let first = &self.vertices[1] - &self.vertices[0];
        let second = &self.vertices[3] - &self.vertices[0];

        first * second
    }
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Tuple<&'a Point3D, &'a Point3D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 4 {
            return None;
        }

        let l = &self.square.vertices[ self.idx         ];
        let r = &self.square.vertices[(self.idx + 1) % 4];
        self.idx += 1;
        Some(Tuple::new(l, r))
    }
}
