macro_rules! render {
    (
        $canvas:expr,
        $texture_creator:expr,
        $font:expr,
        $is_typing:expr,
        $chat_messages:expr
    ) => {
        if $is_typing {
            $canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,150));
            $canvas.fill_rect(sdl2::rect::Rect::new(968, 0, 312, 504)).unwrap();

            $canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,200));
            $canvas.fill_rect(sdl2::rect::Rect::new(968, 504, 312, 24)).unwrap();
        }

        let mut full_chat_txt = "".to_owned();

        let mut message = 0;
        loop {
            if message == $chat_messages.len() {
                break;
            }

            let formatted_msg = "<".to_owned() + &$chat_messages[message].nick + "> " + &$chat_messages[message].message;
            let mut formatted_msg_iter = formatted_msg.chars().enumerate();
            loop {
                let character = formatted_msg_iter.next();
                if character.is_none() {
                    break;
                }
                let character = character.unwrap();
                if character.0 > 26 {
                    full_chat_txt += "\n";
                }
                full_chat_txt += &character.1.to_string();
            }

            full_chat_txt += "\n";
            message += 1;
        }

        if full_chat_txt.len() > 0 {
            let mut splitted_full_chat_txt = full_chat_txt.split('\n');
            let mut max_len = 0;
            let mut line_amount = 0;
            loop {
                let line = splitted_full_chat_txt.next();
                if line.is_none() {
                    break;
                }
                let line = line.unwrap();
                if line.len() > max_len {
                    max_len = line.len();
                }
                line_amount += 1;
            }

            let width = max_len as u32 * 12;
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&full_chat_txt).blended_wrapped(sdl2::pixels::Color::RGB(255,255,255), 312).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(1280 - width as i32, 504 - (24 * line_amount), width, (504 - (24 * line_amount)) as u32))).unwrap();
        }
    }
}

pub(crate) use render;
