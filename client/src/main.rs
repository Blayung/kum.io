mod data;
mod frame;

use std::str::FromStr;

pub fn main() {
    // INITIALIZATION
    // Sdl2
    let sdl_context = sdl2::init().unwrap();
    let sdl_ttf_context = sdl2::ttf::init().unwrap();
    let sdl_ttf_font = sdl_ttf_context.load_font(std::path::Path::new("monospace.medium.ttf"), 128).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut canvas = video_subsystem.window("Kum.io client", 600, 600).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // The http client
    data::http_client::init();

    // Textures
    let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Couldn't connect to server!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Invalid IP!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("This nick is already taken!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();

    // Pls, leave these comments, cause my 16yo laptop cannot handle textures bigger than 2048x2048 :(
    //let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("C").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    //let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("I").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    //let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("T").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();

    // Other
    let mut frame_start: std::time::Instant; 

    let mut game_stage: u8 = 0;

    let mut input = "193.107.8.49:8888".to_owned();
    let mut flickering_cursor: u8 = 0;
    let mut letter_pressed: Option<char>;
    let mut shift_pressed = false;

    let mut forward_pressed = false;
    let mut right_pressed = false;
    let mut backward_pressed = false;
    let mut left_pressed = false;

    // The main loop
    'main_loop: loop {
        frame_start = std::time::Instant::now();        

        frame::main::frame!(
            'main_loop,
            canvas,
            event_pump,
            texture_creator,
            sdl_ttf_font,
            server_conn_err_texture,
            invalid_ip_texture,
            nick_taken_texture,
            game_stage,
            input,
            flickering_cursor,
            letter_pressed,
            shift_pressed,
            forward_pressed,
            right_pressed,
            backward_pressed,
            left_pressed
        );

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 16666666).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }

    // Pre-quitting code
    // Logging out
    if game_stage == 2 {
        data::http_client::get().post(data::server_ip::get().to_owned() + "/log_out").body((&data::credentials::get().0).to_owned().to_owned() + "," + &data::credentials::get().1).send().unwrap();
    }
}
