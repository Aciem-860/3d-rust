extern crate sdl2;

mod point;
mod rotation;
mod square;
mod tuple;
mod cube;

use point::*;
use rotation::*;
use square::*;
use tuple::*;
use cube::*;

use lazy_static::lazy_static;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::sync::{Arc, Mutex};
use std::time::Duration;

macro_rules! rad {
    ($angle:expr) => {
        $angle * std::f32::consts::PI / 180.0
    };
}

const WINDOW_WIDTH: u32 = 1500;
const WINDOW_HEIGHT: u32 = 1000;
const SQUARE_SIZE: u32 = 1;
const WIDTH: u32 = WINDOW_WIDTH / SQUARE_SIZE;
// const HEIGHT: u32 = WINDOW_HEIGHT / SQUARE_SIZE;
const FOV: f32 = rad!(75.0);
const STEP: i32 = 20;

lazy_static! {
    static ref PLAYER: Arc<Mutex<Point3D>> = Arc::new(Mutex::new(Point3D::new(0., 0., 0.)));
}

pub fn main() {
    // let vertices0 = [
    //     Point3D::new(20., 20., 150.),
    //     Point3D::new(60., 20., 150.),
    //     Point3D::new(60., 60., 150.),
    //     Point3D::new(20., 60., 150.),
    // ];

    // let vertices1 = [
    //     Point3D::new(20., 20., 125.),
    //     Point3D::new(60., 20., 125.),
    //     Point3D::new(60., 60., 125.),
    //     Point3D::new(20., 60., 125.),
    // ];

    // let vertices2 = [
    //     Point3D::new(20., 20., 125.),
    //     Point3D::new(60., 20., 125.),
    //     Point3D::new(60., 20., 150.),
    //     Point3D::new(20., 20., 150.),
    // ];

    // let mut squares: Vec<Square> = vec![
    //     Square::new(&vertices0, &Color::GREEN),
    //     Square::new(&vertices1, &Color::BLUE),
    //     Square::new(&vertices2, &Color::RED),
    // ];

    let corner = Point3D::new(20., 20., 150.);
    let cube = Cube::new(&corner, Color::CYAN, 20.);
    let mut squares: Vec<Square> = cube.into();

    let mut cur_rotation: Rotation3 = Rotation3::new(0., 0., 0.);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("my 3d engine", WINDOW_WIDTH, WINDOW_HEIGHT)
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

        let rot_revert = &cur_rotation.revert();
        let (dir_x, dir_y, dir_z) = (
            Point3D::X.rotate(&Point3D::ZERO, rot_revert),
            Point3D::Y.rotate(&Point3D::ZERO, rot_revert),
            Point3D::Z.rotate(&Point3D::ZERO, rot_revert),
        );

        for s in squares.iter_mut() {
            let mut vertices = [Point3D::ZERO; 4];

            for (i, v) in s.vertices.iter().enumerate() {
                vertices[i] = v.rotate(&p, &cur_rotation);
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
                    cur_rotation += Rotation3::new_axis(rad!(1.0), Rotation::Y);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    cur_rotation -= Rotation3::new_axis(rad!(1.0), Rotation::Y);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    cur_rotation += Rotation3::new_axis(rad!(1.0), Rotation::X);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    cur_rotation -= Rotation3::new_axis(rad!(1.0), Rotation::X);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
