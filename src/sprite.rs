use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub trait Sprite {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub trait TexturedSprite {
    fn draw(&self, canvas: &mut Canvas<Window>, texture: &mut Texture);
}
