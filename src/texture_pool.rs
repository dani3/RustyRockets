use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::video::{WindowContext, Window};

const HEIGHT: u32 = 15;
const WIDTH: u32 = 3;

const POPULATION_SIZE: usize = 300;

pub struct Cache<'s> {
    pub textures: Vec<Texture<'s>>,
}

impl<'s> Cache<'s> {
    pub fn new(texture_creator: &'s TextureCreator<WindowContext>) -> Self {
        let mut textures = vec![];
        for _ in 0 .. POPULATION_SIZE {
            let texture = texture_creator.create_texture_target(None, WIDTH, HEIGHT)
                .expect("Failed to create a texture");

            textures.push(texture)
        }

        Cache {
            textures
        }
    }
}

pub struct TexturePool {
    pub texture_creator: TextureCreator<WindowContext>
}

impl TexturePool {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        let texture_creator : TextureCreator<_> = canvas.texture_creator();

        TexturePool {
            texture_creator
        }
    }
}