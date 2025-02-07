use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use sdl2::{
    event::{Event, WindowEvent},
    init,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    rect::Rect,
    render::BlendMode,
    surface::Surface,
};

use crate::{state::*, types::Colour};

pub fn epic() {
    let context = init().unwrap();

    let mut state = State::new(&context);

    let mut event_pump = context.event_pump().unwrap();

    state
        .window
        .surface(&event_pump)
        .unwrap()
        .update_window()
        .unwrap();

    let mut running = true;

    let (mut mouse_x, mut mouse_y) = (0, 0);

    let mut colour = Colour {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

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
                    println!("new size: {:?}", state.window.size());
                }
                _ => {}
            }
        }

        let r = (mouse_x * 255 / state.width) as u8;
        let g = (mouse_y * 255 / state.height) as u8;

        colour.r = r;
        colour.g = g;

        state.clear(colour);

        let mut ws = state.window.surface(&event_pump).unwrap();
        let rect: Rect = state.create_rect();
        state.draw_surface.blit(rect, &mut ws, rect).unwrap();

        ws.update_window().unwrap();

        println!("elapsed: {}", now.elapsed().as_millis());

        sleep(Duration::from_millis(1000 / 60));
    }
}
