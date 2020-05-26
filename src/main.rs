use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use indicatif::{ProgressBar, ProgressStyle};

mod constants;
mod dna;
mod obstacle;
mod population;
mod rocket;
mod sprite;
mod target;
mod texture_pool;

use constants::*;
use obstacle::Obstacle;
use population::Population;
use sprite::Sprite;
use target::Target;
use texture_pool::{TextureManager, TexturePool};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rusty Rockets", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();

    // Create progress bar
    let mut pb = ProgressBar::new(LIFESPAN as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg} ({eta})")
        .progress_chars("=>-"));

    // Create the first population
    let mut population = Population::new();
    // Create the target
    let target = Target::new();
    // Create an obstacle
    let obstacle = Obstacle::new(
        Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2),
        SCREEN_WIDTH as u32 - (SCREEN_WIDTH as u32 / 3),
        25,
    );

    // Create a texture pool
    let texture_manager = TextureManager::new(&canvas);
    let mut texture_pool = TexturePool::new(&texture_manager.texture_creator);

    println!();

    let mut count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(40, 44, 52));
        canvas.clear();

        // Draw the target
        obstacle.draw(&mut canvas);
        target.draw(&mut canvas);

        if count == LIFESPAN {
            count = 0;

            pb.finish_with_message("finished");
            pb = ProgressBar::new(LIFESPAN as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg} ({eta})")
                .progress_chars("=>-"));

            population.evaluate(&target, &obstacle);
            population.natural_selection();
        } else {
            pb.inc(1);

            // Update and draw the population
            population.run(&mut canvas, &target, &obstacle, &mut texture_pool);

            count += 1;
        }

        canvas.present();

        thread::sleep(time::Duration::from_millis(5));
    }
}
