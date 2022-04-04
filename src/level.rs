use crate::kmath::*;
use crate::renderer::*;
use crate::rect::*;

#[derive(Clone)]
pub struct Level {
    tiles: Vec<Vec3>,
    w: i32,
    h: i32,

    tape: Vec<Vec3>,
    player: (i32, i32),
    powerups: Vec<(i32, i32, i32)>,
}

impl Level {
    pub fn new(w: i32, h: i32) -> Level {
        Level {
            w,
            h,
            tiles: vec![Vec3::new(0.0, 0.0, 0.0);( w*h) as usize],
            tape: vec![Vec3::new(1.0, 0.0, 0.0)],
            player: (0,0),
            powerups: vec![],
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, colour: Vec3) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("set tile out of bounds");
        }

        self.tiles[(x * self.w + y) as usize] = colour;
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Vec3 {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("get tile out of bounds");
        }

        self.tiles[(x * self.w + y) as usize]
    }

    pub fn resize(&mut self, new_w: i32, new_h: i32) {
        let mut new_tiles = vec![Vec3::new(0.0, 0.0, 0.0); (new_w*new_h) as usize];
        for i in 0..new_w.min(self.w) {
            for j in 0..new_h.min(self.h) {
                new_tiles[(i*new_w + j) as usize] = self.tiles[(i*self.w + j) as usize];
            }
        }
        self.w = new_w;
        self.h = new_h;
        self.tiles = new_tiles;
    }

    pub fn draw(&self, renderer: &mut Renderer, rect: Rect) {
        renderer.draw_rect(rect, Vec3::new(0.2, 0.2, 0.2), 50.0);
        for i in 0..self.w {
            for j in 0..self.h {
                renderer.draw_rect(rect.dilate(-0.003).grid_child(i, j, self.w, self.h).dilate(-0.003), self.get_tile(i, j), 100.0)
            }
        }
    }
}