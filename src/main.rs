use indicatif::{ProgressBar, ProgressStyle};
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::cell::RefCell;
use std::path::Path;

mod background;
mod constants;
mod dna;
mod drawer;
mod obstacle;
mod population;
mod rocket;
mod sprite;
mod target;

use background::Background;
use constants::*;
use drawer::{Drawer, TexturePool};
use obstacle::Obstacle;
use population::Population;
use target::Target;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let drawer = RefCell::new(Drawer::new(&sdl_context));
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create progress bar
    let mut pb = ProgressBar::new(LIFESPAN as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg} ({eta})")
        .progress_chars("=>-"));
    println!();

    // Create the first population
    let mut population = Population::new(POPULATION_SIZE, POPULATION_ORIGIN_X, POPULATION_ORIGIN_Y);
    // Create the target
    let target = Target::new(
        Point::new(TARGET_ORIGIN_X, TARGET_ORIGIN_Y),
        TARGET_WIDTH,
        TARGET_HEIGHT,
    );
    // Create an obstacle
    let obstacle = Obstacle::new(
        Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2),
        SCREEN_WIDTH as u32 - (SCREEN_WIDTH as u32 / 3),
        75,
    );
    // Create the background
    let background = Background::new(SCREEN_HEIGHT as u32, SCREEN_WIDTH as u32);

    // Load the textures
    let txc = &drawer.borrow().texture_creator;
    let mut txp = TexturePool::new();
    txp.add(
        background.name.clone(),
        &txc,
        background.height,
        background.width,
        Some(Path::new("./src/res/textures/background.jpg")),
    );
    txp.add(
        target.name.clone(),
        &txc,
        target.height,
        target.width,
        Some(Path::new("./src/res/textures/target.png")),
    );
    txp.add(
        obstacle.name.clone(),
        &txc,
        obstacle.height,
        obstacle.width,
        None,
    );

    for rocket in &population.rockets {
        txp.add(
            rocket.name.clone(),
            &txc,
            rocket.height,
            rocket.width,
            Some(Path::new("./src/res/textures/rocket.png")),
        );
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

        // Draw the background
        if let Some(ref mut texture) = txp.textures.get_mut(&background.name) {
            drawer.borrow().draw_sprite(&background, texture);
        }

        // Draw the target
        if let Some(ref mut texture) = txp.textures.get_mut(&obstacle.name) {
            drawer.borrow().draw_sprite(&obstacle, texture);
        }

        // Draw the obstacle
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
    }
}
