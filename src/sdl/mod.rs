use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use anyhow;

use super::framebuffer;
use super::gameboy;
use super::palette;


mod rate_limiter;
mod canvas;

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
    timer: sdl2::TimerSubsystem,
}


impl SDL {
    pub fn new() -> anyhow::Result<SDL> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let scale = 4;

        let window = video_subsystem
            .window("Gameboy", 160 * scale, 144 * scale)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().software().build()?;

        canvas.set_scale(scale as f32, scale as f32).unwrap();

        let timer = sdl_context.timer().unwrap();

        Ok(SDL {
            state: State::Running,
            canvas: canvas,
            sdl_context: sdl_context,
            timer: timer,
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

    pub fn start(&mut self, gameboy: &mut gameboy::Gameboy) {
        let mut framebuffer: framebuffer::Framebuffer = [palette::Shade::White; 23040];

        let mut rate_limiter = rate_limiter::new(60);

        'mainloop: loop {
            match self.state {
                State::InstructionAdvance => {
                    gameboy.next_instruction(&mut framebuffer);

                    canvas::draw(&mut self.canvas, &framebuffer);

                    self.canvas.present();
                    self.state = State::Paused;
                }
                State::FrameAdvance => {
                    gameboy.next_frame(&mut framebuffer);

                    canvas::draw(&mut self.canvas, &framebuffer);

                    self.canvas.present();
                    self.state = State::Paused;
                }
                State::Running => {
                    gameboy.next_frame(&mut framebuffer);

                    canvas::draw(&mut self.canvas, &framebuffer);

                    self.canvas.present();
                }
                State::Paused => {
                    self.canvas.clear();
                    canvas::draw(&mut self.canvas, &framebuffer);
                    self.canvas.present();
                }
            }

            rate_limiter.limit(&mut self.timer);

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
