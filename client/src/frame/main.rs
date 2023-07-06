// Main frame macro
macro_rules! frame {
    (
        // All the varaiables passed from the main function
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $server_conn_err_texture:expr,
        $invalid_ip_texture:expr,
        $nick_taken_texture:expr,
        $game_stage:expr,
        $input:expr,
        $flickering_cursor:expr,
        $letter_pressed:expr,
        $shift_pressed:expr,
        $forward_pressed:expr,
        $right_pressed:expr,
        $backward_pressed:expr,
        $left_pressed:expr
    ) => {
        match $game_stage {
            // Typing the ip in screen's frame
            0 => frame::typing_ip::frame!(
                $main_loop,
                $canvas,
                $event_pump,
                $texture_creator,
                $sdl_ttf_font,
                $server_conn_err_texture,
                $invalid_ip_texture,
                $game_stage,
                $input,
                $flickering_cursor
            );,
            // Typing the nick in screen's frame
            1 => frame::typing_nick::frame!(
                $main_loop,
                $canvas,
                $event_pump,
                $texture_creator,
                $sdl_ttf_font,
                $nick_taken_texture,
                $game_stage,
                $input,
                $flickering_cursor,
                $letter_pressed,
                $shift_pressed
            );,
            // Main game's frame
            2 => frame::main_game::frame!(
                $main_loop,
                $canvas,
                $event_pump,
                $forward_pressed,
                $right_pressed,
                $backward_pressed,
                $left_pressed
            );,
            _ => {}
        }
    };
} 

pub(crate) use frame;
