use sdl2::{
    Sdl, pixels::PixelFormatEnum, rect::Rect, render::BlendMode, surface::Surface, video::Window,
};

use crate::types::Colour;

pub struct State<'a> {
    pub width: i32,
    pub height: i32,
    pub window: Window,
    pub draw_surface: Surface<'a>,
}

impl State<'_> {
    pub fn new(context: &Sdl) -> Self {
        let video_subsystem = context.video().unwrap();

        let height = 1080;
        let width = 1920;

        let window: Window = video_subsystem
            .window("triangles", width, height)
            .resizable()
            .build()
            .unwrap();

        let draw_surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();

        let mut state = State {
            width: width as i32,
            height: height as i32,
            window,
            draw_surface,
        };
        state.draw_surface.set_blend_mode(BlendMode::None).unwrap();

        state
    }

    pub fn create_rect(&self) -> Rect {
        Rect::new(0, 0, self.width as u32, self.height as u32)
    }

    pub fn resize(&mut self) {
        println!("{:?}", self.window.size());
    }

    pub fn clear(&self, colour: Colour) {
        // make a u32
        let colour = colour.to_u32();

        unsafe {
            let size = (self.width * self.height) as usize;

            let surface = *self.draw_surface.raw();
            let pixels = surface.pixels as *mut u32;

            let slice = std::slice::from_raw_parts_mut(pixels, size);

            slice.fill(colour);
        }
    }
}
