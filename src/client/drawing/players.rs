macro_rules! render {
    (
        $canvas:expr,
        $texture_creator:expr,
        $font:expr,
        $player_texture:expr,
        $camera_x_offset:expr,
        $camera_y_offset:expr,
        $players:expr
    ) => {
        $canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,150));
        for player in $players {
            let x = (player.x as i32) - $camera_x_offset;
            let y = (player.y as i32) - $camera_y_offset;
            let text_width = 12 * (player.nick.len() as u32);
            let text_x = x + ((70 - text_width as i32) / 2);
            let text_y = y - 30;
            $canvas.fill_rect(sdl2::rect::Rect::new(text_x - 1, text_y, text_width + 2, 24)).unwrap();
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&player.nick).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(text_x, text_y, text_width, 24))).unwrap();
            $canvas.copy_ex(&$player_texture, None, Some(sdl2::rect::Rect::new(x, y, 70, 70)), player.direction as f64, sdl2::rect::Point::new(35, 35), false, false).unwrap();
        }
    }
}

pub(crate) use render;
