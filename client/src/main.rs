mod data;
mod frame;
mod keep_alive_thread;
mod every_tick_thread;

use std::str::FromStr;
use sdl2::image::LoadTexture;

pub fn main() {
    // INITIALIZATION
    // Sdl2
    let sdl_context = sdl2::init().unwrap();
    let sdl_ttf_context = sdl2::ttf::init().unwrap();
    let sdl_ttf_font = sdl_ttf_context.load_font(std::path::Path::new("./assets/fonts/MonospaceBold.ttf"), 128).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut canvas = video_subsystem.window("Kum.io client", 1280, 720).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // The http client
    data::http_client::init();

    // Textures
    let player_texture = texture_creator.load_texture(std::path::Path::new("./assets/textures/player.png")).unwrap();
    let grass_texture = texture_creator.load_texture(std::path::Path::new("./assets/textures/grass.png")).unwrap();

    let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Couldn't connect to server!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Invalid IP!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("This nick is already taken!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    // Pls, leave these comments, cause my 16yo laptop cannot handle textures bigger than 2048x2048 :(
    //let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("C").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    //let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("I").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    //let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("T").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();

    // Other
    let mut game_stage: u8 = 0;

    let mut input = "193.107.8.49:8888".to_owned();
    let mut cursor: u8 = 17;
    let mut flickering_cursor: u8 = 0;
    let mut shift_pressed = false;
    let mut start_of_error_display = std::time::Instant::now();
    let mut error_displayed = 0;

    let mut server_name = "".to_owned();
    let mut server_name_len = 0;

    let mut debug_menu = false;
    let mut last_elapsed = std::time::Duration::ZERO;

    // The main loop
    'main_loop: loop {
        let frame_start = std::time::Instant::now();        

        frame::main::frame!(
            'main_loop,
            canvas,
            event_pump,
            texture_creator,
            sdl_ttf_font,
            player_texture,
            grass_texture,
            server_conn_err_texture,
            invalid_ip_texture,
            nick_taken_texture,
            game_stage,
            input,
            cursor,
            flickering_cursor,
            shift_pressed,
            start_of_error_display,
            error_displayed,
            server_name,
            server_name_len,
            debug_menu,
            last_elapsed
        );

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 8333333).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        last_elapsed = frame_start.elapsed();
    }

    // Pre-quitting code
    // Logging out
    if game_stage == 2 {
        data::http_client::get().post(data::server_ip::get().to_owned() + "/log_out").body((&data::credentials::get().0).to_owned().to_owned() + "," + &data::credentials::get().1).send().unwrap();
    }
}
