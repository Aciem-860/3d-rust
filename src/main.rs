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
use sdl2::render::Canvas;
use sdl2::sys::{Window, SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SQUARE_SIZE: u32 = 5;
const SQUARE_N_WIDTH: u32 = WINDOW_WIDTH / SQUARE_SIZE;
const SQUARE_N_HEIGHT: u32 = WINDOW_HEIGHT / SQUARE_SIZE;
const WIDTH: u32 = WINDOW_WIDTH / SQUARE_SIZE;
const HEIGHT: u32 = WINDOW_HEIGHT / SQUARE_SIZE;
const FOV: f64 = 1.57; // Field of view (90°)
const STEP: i32 = 5;

lazy_static! {
    static ref PLAYER: Arc<Mutex<Point3D>> = Arc::new(Mutex::new(Point3D::new(
        (WIDTH / 2) as i32,
        (HEIGHT / 2) as i32,
        0
    )));
}

fn is_in_bound(point: &Point2D) -> bool {
    point.x < 0 || point.x > WIDTH as i32 || point.y < 0 || point.y > HEIGHT as i32
}

fn get_rect_from_position(point: &Point2D) -> Option<Rect> {
    if is_in_bound(point) {
        return None;
    }

    Some(Rect::new(
        point.x * SQUARE_SIZE as i32,
        point.y * SQUARE_SIZE as i32,
        SQUARE_SIZE,
        SQUARE_SIZE,
    ))
}

// W     = 40
// THETA = 90
// D     = 20

pub fn main() {
    // let points = vec!{
    //     Point3D::new(20 + 200, 20, 125),
    //     Point3D::new(60 + 200, 20, 125),
    //     Point3D::new(60 + 200, 60, 125),
    //     Point3D::new(20 + 200, 60, 125),
    //     Point3D::new(20 + 200, 20, 150),
    //     Point3D::new(60 + 200, 20, 150),
    //     Point3D::new(60 + 200, 60, 150),
    //     Point3D::new(20 + 200, 60, 150),
    // };

    let vertices0 = [
        Point3D::new(20 + 200, 20, 150),
        Point3D::new(60 + 200, 20, 150),
        Point3D::new(60 + 200, 60, 150),
        Point3D::new(20 + 200, 60, 150),
    ];

    let vertices1 = [
        Point3D::new(20 + 200, 20, 125),
        Point3D::new(60 + 200, 20, 125),
        Point3D::new(60 + 200, 60, 125),
        Point3D::new(20 + 200, 60, 125),
    ];

    let squares: Vec<Square> = vec![
        Square::new(&vertices0, &Color::GREEN),
        Square::new(&vertices1, &Color::BLUE),
    ];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        // Draw the background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for s in squares.iter() {
            canvas.set_draw_color(s.color);

            for vertices in s.iter_pairs() {
                let _ = canvas.draw_line(
                    sdl2::rect::Point::from(&Point2D::from(vertices.first)),
                    sdl2::rect::Point::from(&Point2D::from(vertices.second)),
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
                    p.z += STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    p.z -= STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    p.x += STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    p.x -= STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    p.y += STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    let mut p = PLAYER.lock().unwrap();
                    p.y -= STEP;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
