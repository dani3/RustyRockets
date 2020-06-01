use sdl2::image::LoadTexture;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

use crate::constants::*;
use crate::sprite::Sprite;

pub struct Drawer {
    pub canvas: RefCell<Canvas<Window>>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Drawer {
    pub fn new(ctx: &Sdl) -> Self {
        let vs = ctx.video().unwrap();
        let window = vs
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

        canvas.set_blend_mode(BlendMode::Blend);

        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        Self {
            canvas: RefCell::new(canvas),
            texture_creator,
        }
    }

    pub fn update(&self) {
        self.canvas.borrow_mut().present();
    }

    pub fn draw_sprite<T>(&self, sprite: &T, texture: &mut Texture)
    where
        T: Sprite,
    {
        sprite.draw(&mut self.canvas.borrow_mut(), texture);
    }
}

pub struct TexturePool<'s> {
    pub textures: HashMap<String, Texture<'s>>,
}

impl<'s> TexturePool<'s> {
    pub fn new() -> Self {
        TexturePool {
            textures: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        id: String,
        txc: &'s TextureCreator<WindowContext>,
        w: u32,
        h: u32,
        path: Option<&Path>,
    ) {
        let texture;
        if let Some(image) = path {
            texture = txc
                .load_texture(image)
                .expect("Failed to create a texture with the image");
        } else {
            texture = txc
                .create_texture_target(None, w, h)
                .expect("Failed to create a texture");
        }

        self.textures.insert(id, texture);
    }
}
