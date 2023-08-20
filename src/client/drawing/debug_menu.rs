macro_rules! render {
    (
        $canvas:expr,
        $texture_creator:expr,
        $font:expr,
        $ver_info_texture:expr,
        $server_name:expr,
        $server_name_len:expr,
        $our_player:expr,
        $last_elapsed:expr,
        $game_state:expr
    ) => {
        $canvas.copy(&$ver_info_texture, None, Some(sdl2::rect::Rect::new(5, 5, 350, 20))).unwrap();

        let fps;
        let fps_delay = $last_elapsed.as_nanos();
        if fps_delay == 0 {
            fps = "∞".to_owned();
        } else {
            fps = (1000000000/fps_delay).to_string();
        }

        let ctps;
        let ctps_delay = data::ctps_elapsed::get().as_nanos();
        if ctps_delay == 0 {
            ctps = "∞".to_owned();
        } else {
            ctps = (1000000000/ctps_delay).to_string();
        }

        $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("FPS/CTPS (120/20): ".to_owned() + &fps + "/" + &ctps)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 25, (fps.len() as u32 + ctps.len() as u32 + 19) * 10, 20))).unwrap();

        $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Server name: ".to_owned() + &$server_name)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 45, $server_name_len, 20))).unwrap();

        let x = &$game_state.players[$our_player].x.to_string();
        $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("X: ".to_owned() + x)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 65, (x.len() as u32 + 4) * 10, 20))).unwrap();
        let y = &$game_state.players[$our_player].y.to_string();
        $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Y: ".to_owned() + y)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 85, (y.len() as u32 + 4) * 10, 20))).unwrap();

        let direction = &$game_state.players[$our_player].direction.to_string();
        $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Direction: ".to_owned() + direction)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 105, (direction.len() as u32 + 11) * 10, 20))).unwrap();

        let alive_chat_messages_str = &$game_state.chat_messages.len().to_string();
        $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Alive chat messages: ".to_owned() + alive_chat_messages_str)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 125, (alive_chat_messages_str.len() as u32 + 21) * 10, 20))).unwrap();
    }
}

pub(crate) use render;
