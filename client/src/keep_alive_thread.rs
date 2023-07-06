macro_rules! spawn {
    () => {
        std::thread::spawn(|| {
            let mut keep_alive_start: std::time::Instant;
            loop {
                keep_alive_start = std::time::Instant::now();
                data::http_client::get().post(data::server_ip::get().to_owned()+"/keep_alive").body(data::credentials::get().0.to_owned() + "," + &data::credentials::get().1).send().unwrap();
                std::thread::sleep(std::time::Duration::new(20, 0).checked_sub(keep_alive_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
            }
        });
    };
}

pub(crate) use spawn;
