macro_rules! spawn {
    () => {
        std::thread::spawn(|| {
            let mut tick_start: std::time::Instant;
            loop {
                tick_start = std::time::Instant::now();
            
                // Updating the game state
                data::game_state::update();

                // Getting to_send_data & credentials
                let to_send_data = data::to_send_data::get();
                let credentials = data::credentials::get().clone();
                //println!("{:#?}", to_send_data);

                // Movement
                let move_direction = to_send_data.move_direction;
                if move_direction.is_some() {
                    let running_str;
                    if to_send_data.is_running {
                        running_str = "1";
                    } else {
                        running_str = "0";
                    }
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/move").body(move_direction.unwrap().to_string() + "," + running_str + "," + &credentials.0 + "," + &credentials.1).send().unwrap();
                }

                if to_send_data.rotate.is_some() {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/rotate").body(to_send_data.rotate.unwrap().to_string() + "," + &credentials.0 + "," + &credentials.1).send().unwrap();
                }

                // Setting to_send_data
                data::to_send_data::set(data::to_send_data::ToSendData {
                    move_direction: None,
                    is_running: false,
                    rotate: None
                });
                
                // TPS Limit
                std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
                data::ctps_elapsed::set(tick_start.elapsed());
            }
        });
    };
}

pub(crate) use spawn;
