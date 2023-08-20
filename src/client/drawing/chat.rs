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

        let mut lines: std::vec::Vec<String> = std::vec::Vec::new();

        let mut message = 0;
        loop {
            if message == $chat_messages.len() {
                break;
            }
            lines.push("".to_owned());

            let formatted_msg = format!("{} <{}>", &$chat_messages[message].message, &$chat_messages[message].nick);
            let mut formatted_msg_iter = formatted_msg.chars().enumerate();
            loop {
                let character = formatted_msg_iter.next();
                if character.is_none() {
                    break;
                }
                let character = character.unwrap();
                if character.0 != 0 && character.0 % 26 == 0 {
                    lines.push("".to_owned());
                }
                let last_elem = lines.len() - 1;
                lines[last_elem] += &character.1.to_string();
            }

            message += 1;
        }

        if lines.len() > 0 {
            let start_y = 504 - (lines.len() as i32 * 24);
            
            let mut lines_iter = lines.iter().enumerate();
            loop {
                let line = lines_iter.next();
                if line.is_none() {
                    break;
                }
                let line = line.unwrap();
                let line_width = line.1.len() as u32 * 12;
                $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&line.1).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(1280 - line_width as i32, start_y + (line.0 as i32 * 24), line_width, 24))).unwrap();
            }
        }
    }
}

pub(crate) use render;
