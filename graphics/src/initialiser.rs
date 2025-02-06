use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use sdl2::{
    Sdl,
    event::{Event, WindowEvent},
    init,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    rect::Rect,
    render::BlendMode,
    surface::Surface,
    sys::SDL_WindowFlags,
    video::Window,
};

struct State<'a> {
    width: i32,
    height: i32,
    window: Window,
    draw_surface: Surface<'a>,
}

impl State<'_> {
    fn new(context: &Sdl) -> Self {
        let video_subsystem = context.video().unwrap();

        let height = 600;
        let width = 800;

        let window: Window = video_subsystem
            .window("triangles", 800, 600)
            .position_centered()
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

    fn create_rect(&self) -> Rect {
        Rect::new(0, 0, self.width as u32, self.height as u32)
    }

    fn resize(&mut self) {
        println!("{:?}", self.window.size());
    }

    fn fill(&self, colour: u32) {
        unsafe {
            let size = (self.width * self.height) as usize;

            let surface = *self.draw_surface.raw();
            let pixels = surface.pixels as *mut u32;

            let slice = std::slice::from_raw_parts_mut(pixels, size);

            slice.fill(colour);
        }
    }
}

pub fn epic() {
    let now = Instant::now();

    let context = init().unwrap();
    let mut state = State::new(&context);

    let mut event_pump = context.event_pump().unwrap();

    let mut running = true;

    println!("elapsed: {}", now.elapsed().as_millis());
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                    break;
                }
                Event::MouseButtonUp { .. } => println!("mouse click!"),
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    state.window.maximize();
                    state.resize();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => {
                    running = false;
                    break;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    let surface =
                        Surface::new(width as u32, height as u32, PixelFormatEnum::RGBA32).unwrap();
                    state.draw_surface = surface;
                    state.width = width;
                    state.height = height;
                    state.draw_surface.set_blend_mode(BlendMode::None).unwrap();
                }
                _ => {}
            }
        }

        let colour = 0xFF_FF_DF_DF;
        state.fill(colour);

        let ws = state.window.surface(&event_pump).unwrap();
        let rect: Rect = state.create_rect();
        ws.blit(rect, &mut state.draw_surface, rect)
            .unwrap()
            .unwrap();

        ws.update_window().unwrap();
        sleep(Duration::from_millis(1000 / 60));
    }
}
