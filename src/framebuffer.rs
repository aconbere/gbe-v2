use crate::palette::Shade;

type FB = [Shade; 23040];

fn zero() -> FB {
    [Shade::White; 23040]
}

pub struct Framebuffer {
    buffer: FB
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer {
            buffer: zero(),
        }
    }

    pub fn reset(&mut self) {
        self.buffer = zero()
    }

    pub fn set(&mut self, a: usize, v: Shade) {
        self.buffer[a] = v
    }

    pub fn get(&self, a: usize) -> Shade {
        self.buffer[a]
    }
}
