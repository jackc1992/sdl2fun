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
    video::Window,
};

#[inline]
fn minf(f1: f32, f2: f32) -> f32 {
    if f1 < f2 { f1 } else { f2 }
}

#[inline]
fn maxf(f1: f32, f2: f32) -> f32 {
    if f1 > f2 { f1 } else { f2 }
}

struct State<'a> {
    width: i32,
    height: i32,
    window: Window,
    draw_surface: Surface<'a>,
}

impl State<'_> {
    fn new(context: &Sdl) -> Self {
        let video_subsystem = context.video().unwrap();

        let height = 1080;
        let width = 1920;

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

    fn fill(&self, r: f32, g: f32, b: f32, a: f32) {
        debug_assert!(r <= 1.0);
        debug_assert!(g <= 1.0);
        debug_assert!(b <= 1.0);
        debug_assert!(a <= 1.0);
        // truncate values to 255
        let r = maxf(0.0f32, minf(255.0, r * 255.0)) as u8;
        let g = maxf(0.0f32, minf(255.0, g * 255.0)) as u8;
        let b = maxf(0.0f32, minf(255.0, b * 255.0)) as u8;
        let a = maxf(0.0f32, minf(255.0, a * 255.0)) as u8;

        // make a u32
        let colour = u32::from_le_bytes([r, g, b, a]);

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
    let context = init().unwrap();
    let mut state = State::new(&context);

    let mut event_pump = context.event_pump().unwrap();

    let mut running = true;

    let (mut mouse_x, mut mouse_y) = (0, 0);

    while running {
        let now = Instant::now();
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
                Event::MouseMotion { x, y, .. } => {
                    // relative to the top left of the screen!
                    mouse_x = x;
                    mouse_y = y;
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

        let r = mouse_x as f32 / state.width as f32;
        let g = mouse_y as f32 / state.height as f32;

        state.fill(r, g, 1.0, 1.0);

        let mut ws = state.window.surface(&event_pump).unwrap();
        let rect: Rect = state.create_rect();
        state.draw_surface.blit(rect, &mut ws, rect).unwrap();

        ws.update_window().unwrap();

        println!("elapsed: {}", now.elapsed().as_millis());
        sleep(Duration::from_millis(1000 / 60));
    }
}
