macro_rules! spawn {
    () => {
        std::thread::spawn(|| {
            let mut tick_start: std::time::Instant;
            loop {
                tick_start = std::time::Instant::now();
            
                data::game_state::update();

                let to_send_data = data::to_send_data::get();
                let credentials = data::credentials::get().clone();
                //println!("{:#?}", to_send_data);

                if to_send_data.mov_run {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/mov_run").body("".to_owned() + &credentials.0 + "," + &credentials.1).send().unwrap();
                }
                if to_send_data.mov_forward {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/mov_forward").body("".to_owned() + &credentials.0 + "," + &credentials.1).send().unwrap();
                }
                if to_send_data.mov_backward {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/mov_backward").body("".to_owned() + &credentials.0 + "," + &credentials.1).send().unwrap();
                }
                if to_send_data.mov_left {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/mov_left").body("".to_owned() + &credentials.0 + "," + &credentials.1).send().unwrap();
                }
                if to_send_data.mov_right {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/mov_right").body("".to_owned() + &credentials.0 + "," + &credentials.1).send().unwrap();
                }
                if to_send_data.mov_rotate.is_some() {
                    data::http_client::get().post(data::server_ip::get().to_owned()+"/mov_rotate").body(to_send_data.mov_rotate.unwrap().to_string() + "," + &credentials.0 + "," + &credentials.1).send().unwrap();
                }

                data::to_send_data::set(data::to_send_data::ToSendData {
                    mov_run: false,
                    mov_forward: false,
                    mov_backward: false,
                    mov_left: false,
                    mov_right: false,
                    mov_rotate: None
                });
                
                std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
                data::ctps_elapsed::set(tick_start.elapsed());
            }
        });
    };
}

pub(crate) use spawn;
