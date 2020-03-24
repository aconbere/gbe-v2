use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::ttf;
use sdl2::rect::Point;
use std::path::Path;

use super::palette;
use super::framebuffer;

pub struct DebugText<'a, 'b> {
    font: ttf::Font<'a, 'b>,
    target: Rect,
}


impl <'a, 'b> DebugText <'a, 'b> {
    pub fn new(ttf_context:&'a ttf::Sdl2TtfContext) -> DebugText <'a, 'b> {

        let font = ttf_context.load_font(Path::new("./fonts/Roboto-Black.ttf"), 128).unwrap();

        let target = Rect::new(10, 10, 80, 20);

        DebugText {
            font: font,
            target: target,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, s: &str) {
        let texture_creator = canvas.texture_creator();
        let surface = self.font.render(s)
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        canvas.copy(&texture, None, Some(self.target)).unwrap();
    }
}

/* For each pixel in the frambuffer render the palette shade into a point of
 * a specific color on the canvas.
 */
pub fn draw(
    canvas: &mut Canvas<Window>,
    framebuffer: &framebuffer::Framebuffer
) {
    for x in 0..160 {
        for y in 0..144 {
            let i = (y * 160) + x;
            match framebuffer[i] {
                palette::Shade::White => {
                    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255))
                }
                palette::Shade::LightGrey => {
                    canvas.set_draw_color(Color::RGBA(211, 211, 211, 255))
                }
                palette::Shade::DarkGrey => {
                    canvas.set_draw_color(Color::RGBA(169, 169, 169, 255))
                }
                palette::Shade::Black => {
                    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255))
                }
            }
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
}

