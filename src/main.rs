use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;

mod rocket;

use rocket::Rocket;

const SCREEN_WIDTH: isize  = 1200;
const SCREEN_HEIGHT: isize = 800;

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

    let mut rocket =
        Rocket::new(&mut canvas, Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32));

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

        rocket.update();
        rocket.show(&mut canvas);

        canvas.present();
    }
}
