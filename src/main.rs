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
mod obstacle;

use population::Population;
use target::Target;
use obstacle::Obstacle;
use constants::*;

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
    let mut population = Population::new(&canvas);

    // Create the target
    let mut target = Target::new(&canvas);

    let obstacle = Obstacle::new(Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2), SCREEN_WIDTH as u32 - (SCREEN_WIDTH as u32 / 3), 25);

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
        obstacle.show(&mut canvas);
        target.show(&mut canvas);

        if count == LIFESPAN {
            count = 0;

            population.evaluate(&target);
            population.natural_selection(&canvas);

        } else {
            // Update and draw the population
            population.run(&mut canvas, &target, &obstacle);

            count += 1;
        }

        canvas.present();

        thread::sleep(time::Duration::from_millis(10));
    }
}
