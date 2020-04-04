use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

use crate::palette::Shade;
use crate::framebuffer::Framebuffer;
use crate::cpu::CPU;

use anyhow;
use rate_limiter::RateLimiter;

mod rate_limiter;


const SCALE:u32 = 4;

#[derive(PartialEq, Eq)]
enum State {
    Running,
    Paused,
    FrameAdvance,
    InstructionAdvance,
}

pub struct SDL {
    state: State,
    canvas: Canvas<Window>,
    sdl_context: sdl2::Sdl,
}


impl SDL {
    pub fn new() -> anyhow::Result<SDL> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let scale = 4;

        let window = video_subsystem
            .window("Gameboy", ((160 + 3) * SCALE) + 256, (144 + 2) * SCALE)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().software().build()?;

        canvas.set_scale(scale as f32, scale as f32).unwrap();

        Ok(SDL {
            state: State::Running,
            canvas: canvas,
            sdl_context: sdl_context,
        })
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode: Option::Some(Keycode::Space), ..} => self.toggle_paused(),
            Event::KeyDown { keycode: Option::Some(Keycode::Right), ..} => self.state = State::FrameAdvance,
            Event::KeyDown { keycode: Option::Some(Keycode::Down), ..} => self.state = State::InstructionAdvance,
            _ => {}
        }
    }

    fn toggle_paused(&mut self) {
        match self.state {
            State::Paused => self.state = State::Running,
            _ => self.state = State::Paused,
        }
    }

    /* For each pixel in the framebuffer render the palette shade into a point of
     * a specific color on the canvas.
     */
    pub fn draw_frame(&mut self, origin_x:i32, origin_y: i32, framebuffer: &Framebuffer) {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        self.canvas.draw_rect(Rect::new(origin_x, origin_y, 162, 146)).unwrap();
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));

        for x in 0..160 {
            for y in 0..144 {
                let i = (y * 160) + x;
                let v = framebuffer.get(i);
                match v {
                    Shade::White => {
                        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255))
                    }
                    Shade::LightGrey => {
                        self.canvas.set_draw_color(Color::RGBA(211, 211, 211, 255))
                    }
                    Shade::DarkGrey => {
                        self.canvas.set_draw_color(Color::RGBA(169, 169, 169, 255))
                    }
                    Shade::Black => {
                        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255))
                    }
                }
                self.canvas.draw_point(Point::new(x as i32 + origin_x + 1, y as i32 + origin_y + 1)).unwrap();
            }
        }
    }

    pub fn draw_tile_map(&mut self, origin_x: i32, origin_y: i32, buffer: &[[Shade;256];256]) {
        for y in 0..256 {
            for x in 0..256 {
                let shade = buffer[y][x];
                match shade {
                    Shade::White => {
                        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255))
                    }
                    Shade::LightGrey => {
                        self.canvas.set_draw_color(Color::RGBA(211, 211, 211, 255))
                    }
                    Shade::DarkGrey => {
                        self.canvas.set_draw_color(Color::RGBA(169, 169, 169, 255))
                    }
                    Shade::Black => {
                        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255))
                    }
                }
                self.canvas.draw_point(Point::new(x as i32 + origin_x + 1, y as i32 + origin_y + 1)).unwrap();
            }
        }
    }

    pub fn start(&mut self, cpu: &mut CPU) {
        let mut rate_limiter = RateLimiter::new(60);

        'mainloop: loop {
            match self.state {
                State::InstructionAdvance => {
                    cpu.next_instruction();
                    self.draw_frame(0,0, &cpu.framebuffer);

                    self.canvas.present();
                    self.state = State::Paused;
                }
                State::Running | State::FrameAdvance => {
                    cpu.next_frame();

                    self.draw_frame(0,0, &cpu.framebuffer);
                    self.draw_tile_map((SCALE * 162) as i32, 0, &cpu.get_full_background());

                    self.canvas.present();

                    if self.state == State::FrameAdvance {
                        self.state = State::Paused;
                    }
                }
                State::Paused => {
                    self.canvas.clear();
                    self.draw_frame(0,0, &cpu.framebuffer);
                    self.canvas.present();
                }
            }

            rate_limiter.limit();

            let mut events = self.sdl_context.event_pump().unwrap();

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Option::Some(Keycode::Escape), ..  } => {
                        break 'mainloop
                    },
                    _ => self.handle_event(event)
                }
            }
        }
    }
}
