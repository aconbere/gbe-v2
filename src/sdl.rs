use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

use crate::shade::Shade;
use crate::cpu::CPU;
use crate::tile::Tile;

use anyhow;
use rate_limiter::RateLimiter;

mod rate_limiter;


const SCALE:u32 = 4;

#[derive(PartialEq, Eq)]
enum State {
    Running,
    Paused,
    FrameAdvance,
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

        let window = video_subsystem
            .window("Gameboy",
                (160 * SCALE) + 256,
                144 * SCALE
            )
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().software().build()?;

        // let ttf_context = ttf::init()?;
        // let debug_text = canvas::DebugText::new(&ttf_context);


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
    pub fn draw_frame(&mut self, origin_x:i32, origin_y: i32, cpu: &CPU) {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        self.canvas.draw_rect(Rect::new(origin_x, origin_y, 162, 144)).unwrap();

        for y in 0..144 {
            for x in 0..160 {
                let shade = cpu.buffer[y][x];
                self.set_draw_color(shade);

                let rx = (x as i32 + origin_x) * SCALE as i32;
                let ry = (y as i32 + origin_y) * SCALE as i32;

                self.canvas.fill_rect(
                    Rect::new(rx, ry, SCALE, SCALE)
                ).unwrap();
            }
        }
    }

    pub fn set_draw_color(&mut self, shade: Shade) {
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
    }

    pub fn draw_tile_map(&mut self, origin_x: i32, origin_y: i32, cpu: &CPU) {
        for y in 0..256 {
            for x in 0..256 {
                let pixel = cpu.mmu.gpu.buffer[y][x];
                let shade = cpu.mmu.lcd.bg_palette.map(pixel);
                self.set_draw_color(shade);

                self.canvas.draw_point(
                    Point::new(x as i32 + origin_x, y as i32 + origin_y)
                ).unwrap();
            }
        }

        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 126));
        self.canvas.draw_rect(
            Rect::new(
                (cpu.mmu.lcd.scroll_x as i32) + origin_x,
                cpu.mmu.lcd.scroll_y as i32,
                160, 144)
        ).unwrap();
    }

    fn draw_tiles(&mut self, origin_x: i32, origin_y: i32, cpu: &CPU) {
        // 12 rows of tiles
        for iy in 0..12 {
            // read across for 32 tiles per row (256 pixels)
            for ix in 0..32 {
                let tile_index = (iy * 32) + ix;
                let tile = cpu.mmu.gpu.vram.tile_set[tile_index];
                self.draw_tile(
                    origin_x + (ix as i32 * 8),
                    origin_y + (iy as i32  * 8),
                    tile,
                    cpu);
            }
        }
    }

    fn draw_tile(&mut self, origin_x: i32, origin_y: i32, tile: Tile, cpu: &CPU) {
        for y in 0..8 as usize {
            for x in 0..8 as usize {
                let pixel = tile.data[y][x];
                let shade = cpu.mmu.lcd.bg_palette.map(pixel);
                self.set_draw_color(shade);

                self.canvas.draw_point(
                    Point::new(x as i32 + origin_x, y as i32 + origin_y)
                ).unwrap();
            }
        }
    }

    pub fn start(&mut self, cpu: &mut CPU) {
        let mut rate_limiter = RateLimiter::new(60);

        'mainloop: loop {
            match self.state {
                State::Running | State::FrameAdvance => {
                    cpu.next_frame();

                    self.draw_frame(0,0, &cpu);
                    self.draw_tile_map(160*SCALE as i32, 0, &cpu);
                    self.draw_tiles(160*SCALE as i32, 256, &cpu);

                    self.canvas.present();

                    if self.state == State::FrameAdvance {
                        self.state = State::Paused;
                    }
                }
                State::Paused => {
                    self.canvas.clear();
                    self.draw_frame(0,0, &cpu);
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
