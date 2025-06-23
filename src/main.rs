extern crate sdl2;

mod point;
mod square;
mod tuple;

use point::*;
use square::*;
use tuple::*;

use lazy_static::lazy_static;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use std::time::Duration;

macro_rules! rad {
    ($angle:expr) => {
        $angle * std::f32::consts::PI / 180.0
    };
}

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SQUARE_SIZE: u32 = 2;
const SQUARE_N_WIDTH: u32 = WINDOW_WIDTH / SQUARE_SIZE;
const SQUARE_N_HEIGHT: u32 = WINDOW_HEIGHT / SQUARE_SIZE;
const WIDTH: u32 = WINDOW_WIDTH / SQUARE_SIZE;
const HEIGHT: u32 = WINDOW_HEIGHT / SQUARE_SIZE;
const FOV: f32 = rad!(75.0);
const STEP: i32 = 5;

lazy_static! {
    static ref PLAYER: Arc<Mutex<Point3D>> = Arc::new(Mutex::new(Point3D::new(0, 0, 0)));
}

fn is_in_bound(point: &Point2D) -> bool {
    point.x < 0 || point.x > WIDTH as i32 || point.y < 0 || point.y > HEIGHT as i32
}

fn get_rect_from_position(point: &Point2D) -> Option<Rect> {
    if is_in_bound(point) {
        return None;
    }

    Some(Rect::new(
        (WIDTH / 2) as i32 + point.x * SQUARE_SIZE as i32,
        (HEIGHT / 2) as i32 + point.y * SQUARE_SIZE as i32,
        SQUARE_SIZE,
        SQUARE_SIZE,
    ))
}

// W     = 40
// THETA = 90
// D     = 20

pub fn main() {
    let vertices0 = [
        Point3D::new(20, 20, 150),
        Point3D::new(60, 20, 150),
        Point3D::new(60, 60, 150),
        Point3D::new(20, 60, 150),
    ];

    let vertices1 = [
        Point3D::new(20, 20, 125),
        Point3D::new(60, 20, 125),
        Point3D::new(60, 60, 125),
        Point3D::new(20, 60, 125),
    ];

    let vertices2 = [
        Point3D::new(20, 20, 125),
        Point3D::new(60, 20, 125),
        Point3D::new(60, 20, 150),
        Point3D::new(20, 20, 150),
    ];

    let mut squares: Vec<Square> = vec![
        Square::new(&vertices0, &Color::GREEN),
        Square::new(&vertices1, &Color::BLUE),
        Square::new(&vertices2, &Color::RED),
    ];

    let mut cur_rotation: f32 = 0.0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::YELLOW);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Draw the background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let p = {
            let p = PLAYER.lock().unwrap();
            p.clone()
        };

        let dir_x = Point3D::rotate(&Point3D::X, &Point3D::ZERO, Rotation::RotY(cur_rotation));
        let dir_y = Point3D::Y;
        let dir_z = Point3D::rotate(&Point3D::Z, &Point3D::ZERO, Rotation::RotY(cur_rotation));

        dbg!(&dir_z);

        for s in squares.iter_mut() {
            let mut vertices = [Point3D::ZERO; 4];

            for (i, v) in s.vertices.iter().enumerate() {
                vertices[i] = Point3D::rotate(v, &p, Rotation::RotY(cur_rotation));
            }

            let sq = Square::new(&vertices, &s.color);
            canvas.set_draw_color(sq.color);

            for vertices in sq.iter_pairs() {
                let first = vertices.first;
                let second = vertices.second;

                let _ = canvas.draw_line(
                    sdl2::rect::Point::from(&Point2D::from(first)),
                    sdl2::rect::Point::from(&Point2D::from(second)),
                );
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    let d = &dir_z * STEP;
                    *p += d;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    let d = &dir_z * (-STEP);
                    *p += d;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    let d = &dir_x * STEP;
                    *p += d;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    let d = &dir_x * (-STEP);
                    *p += d;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    let d = &dir_y * STEP;
                    *p += d;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    let d = &dir_y * (-STEP);
                    *p += d;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    cur_rotation += rad!(1.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    cur_rotation -= rad!(1.0);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
