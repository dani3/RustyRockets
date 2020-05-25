use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const HEIGHT: u32 = 15;
const WIDTH: u32 = 3;

const POPULATION_SIZE: usize = 300;

pub struct TexturePool<'s> {
    pub textures: Vec<Texture<'s>>,
}

impl<'s> TexturePool<'s> {
    pub fn new(texture_creator: &'s TextureCreator<WindowContext>) -> Self {
        let mut textures = vec![];
        for _ in 0..POPULATION_SIZE {
            let texture = texture_creator
                .create_texture_target(None, WIDTH, HEIGHT)
                .expect("Failed to create a texture");

            textures.push(texture)
        }

        TexturePool { textures }
    }
}

pub struct TextureManager {
    pub texture_creator: TextureCreator<WindowContext>,
}

impl TextureManager {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        TextureManager { texture_creator }
    }
}
