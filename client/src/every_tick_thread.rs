macro_rules! spawn {
    () => {
        std::thread::spawn(|| {
            let mut tick_start: std::time::Instant;
            loop {
                tick_start = std::time::Instant::now();
            
                data::game_state::update();

                let to_send_data = data::to_send_data::get();

                if to_send_data.should_send_run {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/run").body(data::credentials::get().0.clone() + "," + &data::credentials::get().1).send().unwrap();
                }
                if to_send_data.move_direction.is_some() {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/move").body(to_send_data.move_direction.unwrap().to_string() + "," + &data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();
                }
                if to_send_data.direction.is_some() {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/rotate").body(to_send_data.direction.unwrap().to_string() + "," + &data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();
                }

                data::to_send_data::set(data::to_send_data::ToSendData {
                    should_send_run: false,
                    move_direction: None,
                    direction: None
                });
                
                std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
            }
        });
    };
}

pub(crate) use spawn;
