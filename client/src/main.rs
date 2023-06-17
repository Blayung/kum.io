mod game_state;

const FPS_LIMIT: u32 = 60;
const SCREEN_HEIGHT: u32 = 500;
const SCREEN_WIDTH: u32 = 500;
const TICKRATE: u32 = 20;
const SERVER_IP: &str = "http://localhost:8888";

pub fn main() {
    // Normal vars
    //

    // SDL2 Vars
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Kum.io client", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Every-tick thread
    std::thread::spawn(move || {
        let mut tick_start: std::time::Instant;
        loop {
            tick_start = std::time::Instant::now();
        
            game_state::set(serde_json::from_str(&reqwest::blocking::get(SERVER_IP.to_string()+"/get_game_state").unwrap().text().unwrap()).unwrap());
        
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / TICKRATE).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

    // Main loop
    let mut frame_start: std::time::Instant;
    'main_loop: loop {
        frame_start = std::time::Instant::now();

        // Events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {},
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {},
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {},
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {},
                _ => {}
            }
        }

        // Every-frame stuff
        println!("{:#?}", game_state::get());

        // Drawing to the screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        //canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        //for player in &players {
        //    canvas.fill_rect(sdl2::rect::Rect::new((snake_segment.0 as i16 * cell_size as i16) as i32, (snake_segment.1 as i16 * cell_size as i16) as i32, cell_size as u32, cell_size as u32)).unwrap();
        //}

        canvas.present();

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / FPS_LIMIT).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }
}
