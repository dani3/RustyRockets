use std::cell::RefCell;
use std::{thread, time};

use indicatif::{ProgressBar, ProgressStyle};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod constants;
mod dna;
mod drawer;
mod obstacle;
mod population;
mod rocket;
mod sprite;
mod target;

use constants::*;
use drawer::{Drawer, TexturePool};
use obstacle::Obstacle;
use population::Population;
use target::Target;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let drawer = RefCell::new(Drawer::new(&sdl_context));
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create progress bar
    let mut pb = ProgressBar::new(LIFESPAN as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg} ({eta})")
        .progress_chars("=>-"));

    // Create the first population
    let mut population = Population::new(POPULATION_SIZE, POPULATION_ORIGIN_X, POPULATION_ORIGIN_Y);
    // Create the target
    let target = Target::new();
    // Create an obstacle
    let obstacle = Obstacle::new(
        Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2),
        SCREEN_WIDTH as u32 - (SCREEN_WIDTH as u32 / 3),
        25,
    );

    println!();

    let txc = &drawer.borrow().texture_creator;
    let mut txp = TexturePool::new();
    txp.add(target.name.clone(), &txc, target.height, target.width);
    txp.add(obstacle.name.clone(), &txc, obstacle.height, obstacle.width);

    for rocket in &population.rockets {
        txp.add(rocket.name.clone(), &txc, rocket.height, rocket.width);
    }

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

        drawer.borrow().set_color(Color::RGB(40, 44, 52));

        // Draw the target
        let mut x = txp.textures.get_mut(&obstacle.name);
        if let Some(ref mut texture) = x {
            drawer.borrow().draw_sprite(&obstacle, texture);
        }

        if let Some(ref mut texture) = txp.textures.get_mut(&target.name) {
            drawer.borrow().draw_sprite(&target, texture);
        }

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

            for rocket in population.rockets.iter_mut() {
                rocket.update(&target, &obstacle);

                if let Some(ref mut texture) = txp.textures.get_mut(&rocket.name) {
                    drawer.borrow().draw_sprite(rocket, texture);
                }
            }

            count += 1;
        }

        drawer.borrow().update();

        thread::sleep(time::Duration::from_millis(5));
    }
}
