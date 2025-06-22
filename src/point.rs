use crate::{FOV, PLAYER, WIDTH};

pub const POINT3D_ZERO: Point3D = Point3D { x: 0, y: 0, z: 0 };

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
        let player = PLAYER.lock().unwrap();
        let dx: f64 = (value.x - player.x) as f64;
        let dz: f64 = (value.z - player.z) as f64;
        let d: f64 = f64::from(WIDTH) / (2.0 * (FOV / 2.0).tan());

        let new_x: i32 = (d * dx / dz) as i32;

        let dy: f64 = (value.y - player.y) as f64;
        let new_y: i32 = (d * dy / dz) as i32;

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
