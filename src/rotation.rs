use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug)]
pub struct Rotation3 {
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
}

impl Rotation3 {
    pub fn new(rot_x: f32, rot_y: f32, rot_z: f32) -> Rotation3 {
        Rotation3 { rot_x, rot_y, rot_z }
    }

    pub fn new_axis(angle: f32, axis: Rotation) -> Rotation3 {
        match axis {
            Rotation::X => Self::new(angle, 0., 0.),
            Rotation::Y => Self::new(0., angle, 0.),
            Rotation::Z => Self::new(0., 0., angle),
        }
    }

    pub fn revert(&self) -> Rotation3 {
        Rotation3::new(-self.rot_x, -self.rot_y, -self.rot_z)
    }
}

impl Add for &Rotation3 {
    type Output = Rotation3;
    fn add(self, rhs: &Rotation3) -> Rotation3 {
        Rotation3::new(
            self.rot_x + rhs.rot_x,
            self.rot_y + rhs.rot_y,
            self.rot_z + rhs.rot_z
        )
    }
}

impl Sub for &Rotation3 {
    type Output = Rotation3;
    fn sub(self, rhs: &Rotation3) -> Rotation3 {
        Rotation3::new(
            self.rot_x - rhs.rot_x,
            self.rot_y - rhs.rot_y,
            self.rot_z - rhs.rot_z
        )
    }
}

impl Add for Rotation3 {
    type Output = Rotation3;
    fn add(self, rhs: Rotation3) -> Rotation3 {
        &self + &rhs
    }
}

impl Sub for Rotation3 {
    type Output = Rotation3;
    fn sub(self, rhs: Rotation3) -> Rotation3 {
        &self - &rhs
    }
}

impl AddAssign for Rotation3 {
    fn add_assign(&mut self, rhs: Rotation3) {
        *self = &*self + &rhs
    }
}

impl SubAssign for Rotation3 {
    fn sub_assign(&mut self, rhs: Rotation3) {
        *self = &*self - &rhs
    }
}
