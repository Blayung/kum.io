// FRAME FOR THE MAIN GAME SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $player_texture:expr,
        $forward_pressed:expr,
        $right_pressed:expr,
        $backward_pressed:expr,
        $left_pressed:expr
    ) => {
        // Events
        let mut should_send_run = false;

        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => $forward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => $right_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => $backward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => $left_pressed = true,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => $forward_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => $right_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => $backward_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => $left_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } => should_send_run = true,
                _ => {}
            }
        }

        // Every-frame pre-drawing stuff
        let mut to_send_data = data::to_send_data::get();

        if should_send_run {
            to_send_data.should_send_run = true;
        }

        if $forward_pressed && $right_pressed && $left_pressed && $backward_pressed {}
        else if $forward_pressed && $right_pressed && $left_pressed {
            to_send_data.move_direction = Some('6');
        } else if $forward_pressed && $backward_pressed && $left_pressed { 
            to_send_data.move_direction = Some('4');
        } else if $forward_pressed && $backward_pressed && $right_pressed { 
            to_send_data.move_direction = Some('0');
        } else if $backward_pressed && $left_pressed && $right_pressed {
            to_send_data.move_direction = Some('2');
        } else if $forward_pressed && $right_pressed {
            to_send_data.move_direction = Some('7');
        } else if $forward_pressed && $left_pressed {
            to_send_data.move_direction = Some('5');
        } else if $backward_pressed && $right_pressed {
            to_send_data.move_direction = Some('1');
        } else if $backward_pressed && $left_pressed {
            to_send_data.move_direction = Some('3');
        } else if $forward_pressed {
            to_send_data.move_direction = Some('6');
        } else if $right_pressed {
            to_send_data.move_direction = Some('0');
        } else if $left_pressed {
            to_send_data.move_direction = Some('4');
        } else if $backward_pressed {
            to_send_data.move_direction = Some('2');
        }

        data::to_send_data::set(to_send_data);

        //println!("{:#?}", data::game_state::get());

        // Drawing to the screen
        $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        $canvas.clear();

        for player in data::game_state::get().players {
            $canvas.copy(&$player_texture, None, Some(sdl2::rect::Rect::new(player.x as i32, player.y as i32, 60, 60))).unwrap();
        }

        $canvas.present();
    };
} 

pub(crate) use frame;
