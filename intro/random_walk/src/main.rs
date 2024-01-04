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

    fn step_r(&mut self) {
        let mut rng = rand::thread_rng();
        let uni = Uniform::from(0.0..1.0);
        let choice: f32 = uni.sample(&mut rng);
        if choice < 0.4 {
            self.x += 1.0;
        } else if choice < 0.6 {
            self.x -= 1.0;
        } else if choice < 0.8 {
            self.y += 1.0;
        } else {
            self.y -= 1.0;
        }
    }

    fn step_towards_mouse(&mut self, prob: f32, mouse_pos: Vector2) {
        let mut rng = rand::thread_rng();
        let uni = Uniform::from(0.0..1.0);
        let choice: f32 = uni.sample(&mut rng);
        if choice < prob {
            if mouse_pos.x > self.x {
                self.x += 1.0;
            } else {
                self.x -= 1.0;
            }
            if mouse_pos.y > self.y {
                self.y += 1.0;
            } else {
                self.y -= 1.0;
            }
        } else {
            let uni = Uniform::from(-1.0..1.0);
            let stepx: f32 = uni.sample(&mut rng);
            let stepy: f32 = uni.sample(&mut rng);
            self.x += stepx;
            self.y += stepy;
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Random Walk")
        .build();
    rl.set_target_fps(144);

    let mut walker: Walker = Walker::new(rl.get_screen_width()/2, rl.get_screen_height()/2);
    
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::from((0x18u8, 0x18u8, 0x18u8, 0xFFu8)));
    drop(d);
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mouse = d.get_mouse_position();
        
        // walker.step();
        walker.step_towards_mouse(0.5, mouse);
        walker.display(&mut d);
        // d.clear_background(Color::from((0x18u8, 0x18u8, 0x18u8, 0xFFu8)));
    }
}
