extern crate sdl2;

mod emscripten_boilerplate;

use emscripten_boilerplate::setup_mainloop;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Kum.io", 450, 450)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let fps = -1; // call the function as fast as the browser wants to render (typically 60fps)
    let simulate_infinite_loop = 1; // call the function repeatedly
    let mut iteration = 0;
    setup_mainloop(fps, simulate_infinite_loop, move || {
        // example: draw a moving rectangle

        // red background
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        // moving blue rectangle
        iteration = (iteration + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let rect = Rect::new(iteration, 50, 50, 50);
        let _ = canvas.fill_rect(rect);
    })
}
