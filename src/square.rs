use sdl2::pixels::Color;

use crate::{Point3D, POINT3D_ZERO, Tuple};

pub struct Square {
    pub vertices: [Point3D; 4],
    pub color: Color,
}

pub struct SquareIter<'a> {
    square: &'a Square,
    idx: usize,
}

impl Square {
    pub fn new(vertices: &[Point3D; 4], color: &Color) -> Square {
        let mut v: [Point3D; 4] = [POINT3D_ZERO; 4];
        v.clone_from_slice(vertices);
        Square {
            vertices: v,
            color: color.clone(),
        }
    }

    pub fn iter_pairs(&self) -> SquareIter<'_> {
        SquareIter {
            square: self,
            idx: 0,
        }
    }
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Tuple<&'a Point3D, &'a Point3D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx {
            4 => {
                None
            }
            _ => {
                let s = if self.idx == 3 {
                    Some(Tuple::new(
                        &self.square.vertices[3],
                        &self.square.vertices[0]
                    ))
                } else {
                    Some(Tuple::new(
                        &self.square.vertices[self.idx],
                        &self.square.vertices[self.idx + 1],
                    ))
                };

                self.idx += 1;
                s
            }
        }
    }
}
