use crate::{FOV, PLAYER, WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub enum Rotation {
    RotX(f32),
    RotY(f32),
    RotZ(f32),
}

#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn rotate(point: &Point3D, center: &Point3D, rot: Rotation) -> Point3D {
        match rot {
            Rotation::RotX(angle_rad) => Point3D::rotate_x(point, center, angle_rad),
            Rotation::RotY(angle_rad) => Point3D::rotate_y(point, center, angle_rad),
            Rotation::RotZ(angle_rad) => Point3D::rotate_z(point, center, angle_rad),
        }
    }

    fn rotate_x(point: &Point3D, center: &Point3D, angle_rad: f32) -> Point3D {
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let dy = (point.y - center.y) as f32;
        let dz = (point.z - center.z) as f32;

        let rotated_y = dy * c - dz * s;
        let rotated_z = dy * s + dz * c;

        Point3D {
            x: point.x,
            y: (rotated_y + center.y as f32).round() as i32,
            z: (rotated_z + center.z as f32).round() as i32,
        }
    }

    fn rotate_y(point: &Point3D, center: &Point3D, angle_rad: f32) -> Point3D {
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let dx = (point.x - center.x) as f32;
        let dz = (point.z - center.z) as f32;

        let rotated_x = dx * c + dz * s;
        let rotated_z = -dx * s + dz * c;

        Point3D {
            x: (rotated_x + center.x as f32).round() as i32,
            y: point.y,
            z: (rotated_z + center.z as f32).round() as i32,
        }
    }

    fn rotate_z(point: &Point3D, center: &Point3D, angle_rad: f32) -> Point3D {
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let dx = (point.x - center.x) as f32;
        let dy = (point.y - center.y) as f32;

        let rotated_x = dx * c - dy * s;
        let rotated_y = dx * s + dy * c;

        Point3D {
            x: (rotated_x + center.x as f32).round() as i32,
            y: (rotated_y + center.y as f32).round() as i32,
            z: point.z,
        }
    }

    pub const ZERO: Point3D = Point3D { x: 0, y: 0, z: 0 };
    pub const X: Point3D = Point3D { x: 1, y: 0, z: 0 };
    pub const Y: Point3D = Point3D { x: 0, y: 1, z: 0 };
    pub const Z: Point3D = Point3D { x: 0, y: 0, z: 1 };
}

impl Add for &Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Point3D {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<i32> for &Point3D {
    type Output = Point3D;

    fn mul(self, rhs: i32) -> Point3D {
        Point3D {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Point3D {
        &self + &rhs
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Mul<i32> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: i32) -> Point3D {
        &self * rhs
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Point3D {
    fn sub_assign(&mut self, rhs: Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}


#[derive(Debug, Clone)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Point2D {
        Point2D { x, y }
    }
}

impl From<&Point3D> for Point2D {
    fn from(value: &Point3D) -> Point2D {
        let player = {
            let p = PLAYER.lock().unwrap();
            p.clone()
        };
        let dx: f32 = (value.x - player.x) as f32;
        let dz: f32 = (value.z - player.z) as f32;
        let d: f32 = WIDTH as f32 / (2.0 * (FOV / 2.0).tan());

        let new_x: i32 = (WINDOW_WIDTH / 2) as i32 + (d * dx / dz).round() as i32;

        let dy: f32 = (value.y - player.y) as f32;

        let new_y: i32 = (WINDOW_HEIGHT / 2) as i32 + (d * dy / dz).round() as i32;

        Point2D { x: new_x, y: new_y }
    }
}

impl From<Point3D> for Point2D {
    fn from(value: Point3D) -> Point2D {
        Point2D::from(&value)
    }
}

impl From<&Point2D> for sdl2::rect::Point {
    fn from(value: &Point2D) -> sdl2::rect::Point {
        sdl2::rect::Point::new(value.x, value.y)
    }
}
