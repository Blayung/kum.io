// FRAME FOR THE MAIN GAME SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $forward_pressed:expr,
        $right_pressed:expr,
        $backward_pressed:expr,
        $left_pressed:expr
    ) => {
        // Events
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
                _ => {}
            }
        }

        // Every-frame pre-drawing stuff
        let mut to_send_data = data::to_send_data::get();

        if $forward_pressed && $right_pressed && $left_pressed && $backward_pressed {}
        else if $forward_pressed && $right_pressed && $left_pressed {
            to_send_data.move_direction = Some('6');
            data::to_send_data::set(to_send_data);
        } else if $forward_pressed && $backward_pressed && $left_pressed { 
            to_send_data.move_direction = Some('4');
            data::to_send_data::set(to_send_data);
        } else if $forward_pressed && $backward_pressed && $right_pressed { 
            to_send_data.move_direction = Some('0');
            data::to_send_data::set(to_send_data);
        } else if $backward_pressed && $left_pressed && $right_pressed {
            to_send_data.move_direction = Some('2');
            data::to_send_data::set(to_send_data);
        } else if $forward_pressed && $right_pressed {
            to_send_data.move_direction = Some('7');
            data::to_send_data::set(to_send_data);
        } else if $forward_pressed && $left_pressed {
            to_send_data.move_direction = Some('5');
            data::to_send_data::set(to_send_data);
        } else if $backward_pressed && $right_pressed {
            to_send_data.move_direction = Some('1');
            data::to_send_data::set(to_send_data);
        } else if $backward_pressed && $left_pressed {
            to_send_data.move_direction = Some('3');
            data::to_send_data::set(to_send_data);
        } else if $forward_pressed {
            to_send_data.move_direction = Some('6');
            data::to_send_data::set(to_send_data);
        } else if $right_pressed {
            to_send_data.move_direction = Some('0');
            data::to_send_data::set(to_send_data);
        } else if $left_pressed {
            to_send_data.move_direction = Some('4');
            data::to_send_data::set(to_send_data);
        } else if $backward_pressed {
            to_send_data.move_direction = Some('2');
            data::to_send_data::set(to_send_data);
        }

        println!("{:#?}", data::game_state::get());
        println!("{:#?}", data::to_send_data::get());

        // Drawing to the screen
        $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        $canvas.clear();

        $canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for player in data::game_state::get().players {
            $canvas.fill_rect(sdl2::rect::Rect::new(player.x, player.y, 50, 50)).unwrap();
        }

        $canvas.present();
    };
} 

pub(crate) use frame;
