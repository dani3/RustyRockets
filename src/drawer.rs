use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use crate::constants::*;
use crate::sprite::Sprite;

pub struct Drawer {
    canvas: RefCell<Canvas<Window>>,
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

        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .unwrap();

        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        Self {
            canvas: RefCell::new(canvas),
            texture_creator,
        }
    }

    pub fn set_color(&self, color: Color) {
        self.canvas.borrow_mut().set_draw_color(color);
        self.canvas.borrow_mut().clear();
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
    pub textures: Vec<Texture<'s>>,
}

impl<'s> TexturePool<'s> {
    pub fn new() -> Self {
        TexturePool { textures: vec![] }
    }

    pub fn add(&mut self, txc: &'s TextureCreator<WindowContext>, w: u32, h: u32) {
        let texture = txc
            .create_texture_target(None, w, h)
            .expect("Failed to create a texture");

        self.textures.push(texture)
    }
}
