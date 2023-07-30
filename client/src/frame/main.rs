// Main frame macro
macro_rules! frame {
    (
        // All the varaiables passed from the main function
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $player_texture:expr,
        $grass_texture:expr,
        $ver_info_texture:expr,
        $server_conn_err_texture:expr,
        $invalid_ip_texture:expr,
        $nick_taken_texture:expr,
        $game_stage:expr,
        $input:expr,
        $cursor:expr,
        $flickering_cursor:expr,
        $shift_pressed:expr,
        $start_of_error_display:expr,
        $error_displayed:expr,
        $server_name:expr,
        $server_name_len:expr,
        $debug_menu:expr,
        $last_elapsed:expr,
        $is_going_forward:expr,
        $is_going_backward:expr,
        $is_going_left:expr,
        $is_going_right:expr,
        $is_running:expr
    ) => {
        match $game_stage {
            0 => { // Typing the ip in screen's frame
                frame::typing_ip::frame!(
                    $main_loop,
                    $canvas,
                    $event_pump,
                    $texture_creator,
                    $sdl_ttf_font,
                    $ver_info_texture,
                    $server_conn_err_texture,
                    $invalid_ip_texture,
                    $game_stage,
                    $input,
                    $cursor,
                    $flickering_cursor,
                    $start_of_error_display,
                    $error_displayed,
                    $server_name,
                    $server_name_len
                );
            },
            1 => { // Typing the nick in screen's frame
                frame::typing_nick::frame!(
                    $main_loop,
                    $canvas,
                    $event_pump,
                    $texture_creator,
                    $sdl_ttf_font,
                    $ver_info_texture,
                    $nick_taken_texture,
                    $game_stage,
                    $input,
                    $cursor,
                    $flickering_cursor,
                    $start_of_error_display,
                    $error_displayed,
                    $shift_pressed
                );
            },
            2 => { // Main game's frame
                frame::main_game::frame!(
                    $main_loop,
                    $canvas,
                    $event_pump,
                    $texture_creator,
                    $sdl_ttf_font,
                    $player_texture,
                    $grass_texture,
                    $ver_info_texture,
                    $server_name,
                    $server_name_len,
                    $debug_menu,
                    $last_elapsed,
                    $is_going_forward,
                    $is_going_backward,
                    $is_going_left,
                    $is_going_right,
                    $is_running
                );
            },
            _ => {}
        }
    };
} 

pub(crate) use frame;
