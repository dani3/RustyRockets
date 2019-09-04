use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod rocket;
mod population;
mod dna;
mod constants;
mod target;

use population::Population;
use target::Target;
use constants::LIFESPAN;

const SCREEN_WIDTH: isize  = 1200;
const SCREEN_HEIGHT: isize = 800;

const POPULATION_ORIGIN_X: i32 = SCREEN_WIDTH as i32 / 2;
const POPULATION_ORIGIN_Y: i32 = SCREEN_HEIGHT as i32;

const TARGET_ORIGIN_X: i32 = SCREEN_WIDTH as i32 / 2;
const TARGET_ORIGIN_Y: i32 = 50;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rusty Rockets", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();

    // Create the first population
    let mut population =
        Population::new(&canvas, Point::new(POPULATION_ORIGIN_X, POPULATION_ORIGIN_Y));

    // Create the target
    let mut target =
        Target::new(&canvas, Point::new(TARGET_ORIGIN_X, TARGET_ORIGIN_Y));

    let mut count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the target
        target.show(&mut canvas);

        if count == LIFESPAN {
            count = 0;
            population =
                Population::new(&canvas, Point::new(POPULATION_ORIGIN_X, POPULATION_ORIGIN_Y));

        } else {
            // Update and draw the population
            population.run(&mut canvas);
            count += 1;
        }

        canvas.present();

        thread::sleep(time::Duration::from_millis(10));
    }
}
