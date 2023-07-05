mod data;
mod frame;

pub fn main() {
    // Init
    data::http_client::init();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let sdl_ttf_context = sdl2::ttf::init().unwrap();
    let sdl_ttf_font = sdl_ttf_context.load_font(std::path::Path::new("monospace.medium.ttf"), 128).unwrap();

    let window = video_subsystem.window("Kum.io client", 600, 600).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frame_start: std::time::Instant; 

    /*
    let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Couldn't connect to server!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Invalid IP!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("This nick is already taken!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    */

    loop {
        frame_start = std::time::Instant::now(); 

        /*
        macro_rules! basic_drawing {
            (to_draw_objects: std::vec::Vec<ToDrawObj>) => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas.clear();

                for to_draw_obj in to_draw_objects {
                    match to_draw_obj {
                        frame::ToDrawObj::Rectangle(rect, color) => {
                            canvas.set_draw_color(color);
                            canvas.fill_rect(rect).unwrap();
                        },
                        frame::ToDrawObj::DynamicText {text, color, bg_color, x, y, size} => {
                            if bg_color.is_some() {
                                unimplemented!();
                            }

                            canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&text).blended(color).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(x, y, ( size/2 * (text.len() as u32) ).try_into().unwrap(), size))).unwrap();
                        }
                    }
                }
            }
        }
        */

        macro_rules! basic_drawing {
            //() => {
            //    $crate::print!("\n")
            //};
            ($($arg:tt)*) => {{
                //$crate::io::_print($crate::format_args_nl!($($arg)*));
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas.clear();

                for to_draw_obj in $($arg)* {
                    match to_draw_obj {
                        frame::ToDrawObj::Rectangle(rect, color) => {
                            canvas.set_draw_color(color);
                            canvas.fill_rect(rect).unwrap();
                        },
                        frame::ToDrawObj::DynamicText {text, color, bg_color, x, y, size} => {
                            if bg_color.is_some() {
                                unimplemented!();
                            }

                            canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&text).blended(color).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(x, y, ( size/2 * (text.len() as u32) ).try_into().unwrap(), size))).unwrap();
                        }
                    }
                }
            }};
        }

        match frame::frame(event_pump.poll_iter()) {
            frame::FrameReturnData::Quit => break,
            frame::FrameReturnData::DoNothing => {},
            frame::FrameReturnData::Draw(to_draw_objects) => {
                basic_drawing!(to_draw_objects);
            }
            frame::FrameReturnData::DrawAndSleep(to_draw_objects, sleep_duration) => {
                basic_drawing!(to_draw_objects);
                std::thread::sleep(sleep_duration);
            }
        }

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 16666666).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }

    // Logging out
    data::http_client::get().post(data::server_ip::get().to_owned() + "/log_out").body((&data::credentials::get().0).to_owned().to_owned() + "," + &data::credentials::get().1).send().unwrap();
}
