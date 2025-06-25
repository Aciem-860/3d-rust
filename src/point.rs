use crate::{Rotation3, FOV, PLAYER, WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn dot(&self, other: &Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn angle(&self, rhs: &Point3D) -> f32 {
        self.dot(rhs) / (self.norm() * rhs.norm())
    }

    pub fn normalize(&self) -> Point3D {
        let n = self.norm();
        Point3D {
            x: self.x/n,
            y: self.y/n,
            z: self.z/n,
        }
    }

    pub fn rotate(&self, center: &Point3D, rot: &Rotation3) -> Point3D {
        self.rotate_x(center, rot.rot_x)
            .rotate_y(center, rot.rot_y)
            .rotate_z(center, rot.rot_z)
    }

    fn rotate_x(&self, center: &Point3D, angle_rad: f32) -> Point3D {
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let dy = self.y - center.y;
        let dz = self.z - center.z;

        let rotated_y = dy * c - dz * s;
        let rotated_z = dy * s + dz * c;

        Point3D {
            x: self.x,
            y: rotated_y + center.y,
            z: rotated_z + center.z,
        }
    }

    fn rotate_y(&self, center: &Point3D, angle_rad: f32) -> Point3D {
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let dx = self.x - center.x;
        let dz = self.z - center.z;

        let rotated_x = dx * c + dz * s;
        let rotated_z = -dx * s + dz * c;

        Point3D {
            x: rotated_x + center.x,
            y: self.y,
            z: rotated_z + center.z,
        }
    }

    fn rotate_z(&self, center: &Point3D, angle_rad: f32) -> Point3D {
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let dx = self.x - center.x;
        let dy = self.y - center.y;

        let rotated_x = dx * c - dy * s;
        let rotated_y = dx * s + dy * c;

        Point3D {
            x: rotated_x + center.x,
            y: rotated_y + center.y,
            z: self.z,
        }
    }

    pub const ZERO: Point3D = Point3D {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub const X: Point3D = Point3D {
        x: 1.,
        y: 0.,
        z: 0.,
    };
    pub const Y: Point3D = Point3D {
        x: 0.,
        y: 1.,
        z: 0.,
    };
    pub const Z: Point3D = Point3D {
        x: 0.,
        y: 0.,
        z: 1.,
    };
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
            x: rhs as f32 * self.x,
            y: rhs as f32 * self.y,
            z: rhs as f32 * self.z,
        }
    }
}

impl Mul<f32> for &Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f32) -> Point3D {
        Point3D {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul for Point3D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
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

impl Mul<f32> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f32) -> Point3D {
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
        let dx: f32 = value.x - player.x;
        let dz: f32 = value.z - player.z;
        let d: f32 = WIDTH as f32 / (2.0 * (FOV / 2.0).tan());

        let new_x: i32 = (WINDOW_WIDTH / 2) as i32 + (d * dx / dz).round() as i32;
        let dy: f32 = value.y - player.y;
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
