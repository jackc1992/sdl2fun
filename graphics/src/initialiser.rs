use sdl2::init;

fn epic() {
    let context = init().unwrap();
    let subsystem = context.video().unwrap();

    let mut window = subsystem.window("triangles", 800, 600).build().unwrap();

    window.maximize();

    let canvas = window.into_canvas().build().unwrap();
}
