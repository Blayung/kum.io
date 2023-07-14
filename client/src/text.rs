// Leave the width as 0 for automatic scaling (height will be used as overall text size)
pub fn render_dynamic_text(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, font: &sdl2::ttf::Font, text: &str, color: sdl2::pixels::Color, x: i32, y: i32, height: u32, width: u32) {
    if text.len() > 0 {
        if width == 0 {
            canvas.copy(&texture_creator.create_texture_from_surface(font.render(text).blended(color).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(x, y, (height/2) * (text.len() as u32), height))).unwrap();
        } else {
            canvas.copy(&texture_creator.create_texture_from_surface(font.render(text).blended(color).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(x, y, width, height))).unwrap();
        }
    }
}

//pub fn render_text_texture(texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, font: &sdl2::ttf::Font, text: &str, color: sdl2::pixels::Color) -> sdl2::render::Texture {
//    return texture_creator.create_texture_from_surface(font.render(text).blended(color).unwrap()).unwrap();
//}
