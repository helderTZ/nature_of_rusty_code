extern crate raylib;
use raylib::prelude::*;
use rand::distributions::{Distribution, Uniform};

struct Walker {
    x: f32,
    y: f32,
}

impl Walker {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x as f32, y: y as f32 }
    }

    fn display(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, 1.0, Color::WHITE);
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let uni = Uniform::from(-1.0..1.0);
        let stepx: f32 = uni.sample(&mut rng);
        let stepy: f32 = uni.sample(&mut rng);
        self.x += stepx;
        self.y += stepy;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Random Walk")
        .build();

    let mut walker: Walker = Walker::new(rl.get_screen_width()/2, rl.get_screen_height()/2);
    
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::from((0x18u8, 0x18u8, 0x18u8, 0xFFu8)));
    drop(d);
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        walker.step();
        walker.display(&mut d);
        // d.clear_background(Color::from((0x18u8, 0x18u8, 0x18u8, 0xFFu8)));
    }
}
