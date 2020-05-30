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
use population::{Population, POPULATION_SIZE};
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
    let mut population = Population::new();
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
    txp.add(&txc, target.height, target.width);
    txp.add(&txc, target.height, target.width);
    for _ in 0..POPULATION_SIZE {
        txp.add(&txc, 15, 3);
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
        drawer.borrow().draw_sprite(&obstacle, &mut txp.textures[0]);

        drawer.borrow().draw_sprite(&target, &mut txp.textures[1]);

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

            for (i, rocket) in population.rockets.iter_mut().enumerate() {
                rocket.update(&target, &obstacle);
                drawer
                    .borrow()
                    .draw_sprite(rocket, &mut txp.textures[i + 2]);
            }

            count += 1;
        }

        drawer.borrow().update();

        thread::sleep(time::Duration::from_millis(5));
    }
}
