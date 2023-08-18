mod data;
mod frame;
mod keep_alive_thread;
mod every_tick_thread;

use std::str::FromStr;
use sdl2::image::LoadTexture;

fn main() {
    // INITIALIZATION
    // SDL
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(std::include_bytes!("assets/fonts/Cousine-Regular.ttf")).unwrap(), 128).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut canvas = video_subsystem.window("Kum.io client", 1280, 720).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let text_input = video_subsystem.text_input();
    text_input.stop();

    // The http client
    data::http_client::init();

    // Textures
    let player_texture = texture_creator.load_texture_bytes(std::include_bytes!("assets/textures/player.png")).unwrap();
    let grass_texture = texture_creator.load_texture_bytes(std::include_bytes!("assets/textures/grass.png")).unwrap();

    let ver_info_texture = texture_creator.create_texture_from_surface(font.render("Kum.io - Dev build 18/08/2023 10:00").blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap();

    let server_conn_err_texture = texture_creator.create_texture_from_surface(font.render("Couldn't connect to server!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let invalid_ip_texture = texture_creator.create_texture_from_surface(font.render("Invalid IP!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let nick_taken_texture = texture_creator.create_texture_from_surface(font.render("This nick is already taken!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();

    // Other
    let mut game_stage: u8 = 0;

    let mut input = "193.107.8.49:8888".to_owned();
    let mut cursor: u8 = 17;
    let mut flickering_cursor: u8 = 0;
    let mut start_of_error_display = std::time::Instant::now();
    let mut error_displayed = 0;

    let mut server_name = "".to_owned();
    let mut server_name_len = 0;

    let mut debug_menu = false;
    let mut last_elapsed = std::time::Duration::ZERO;
    let mut is_going_forward = false;
    let mut is_going_backward = false;
    let mut is_going_left = false;
    let mut is_going_right = false;
    let mut is_running = false;

    // The main loop
    'main_loop: loop {
        let frame_start = std::time::Instant::now();        

        match game_stage {
            0 => { // Typing the ip in screen's frame
                frame::typing_ip::frame!(
                    'main_loop,
                    canvas,
                    event_pump,
                    texture_creator,
                    font,
                    text_input,
                    ver_info_texture,
                    server_conn_err_texture,
                    invalid_ip_texture,
                    game_stage,
                    input,
                    cursor,
                    flickering_cursor,
                    start_of_error_display,
                    error_displayed,
                    server_name,
                    server_name_len
                );
            },
            1 => { // Typing the nick in screen's frame
                frame::typing_nick::frame!(
                    'main_loop,
                    canvas,
                    event_pump,
                    texture_creator,
                    font,
                    text_input,
                    ver_info_texture,
                    nick_taken_texture,
                    game_stage,
                    input,
                    cursor,
                    flickering_cursor,
                    start_of_error_display,
                    error_displayed
                );
            },
            2 => { // Main game's frame
                frame::main_game::frame!(
                    'main_loop,
                    canvas,
                    event_pump,
                    texture_creator,
                    font,
                    text_input,
                    player_texture,
                    grass_texture,
                    ver_info_texture,
                    server_name,
                    server_name_len,
                    debug_menu,
                    last_elapsed,
                    is_going_forward,
                    is_going_backward,
                    is_going_left,
                    is_going_right,
                    is_running
                );
            },
            _ => {}
        }

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
