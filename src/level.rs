use crate::kmath::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::rect::*;

#[derive(Clone)]
pub struct Level {
    tiles: Vec<Vec3>,
    pub w: i32,
    pub h: i32,

    pub tape: Vec<Vec3>,
    pub tape_cursor: i32,
    pub player: (i32, i32),
    pub alive: bool,
    pub goal: (i32, i32),
    pub powerups: Vec<(i32, i32, i32)>,
}

impl Level {
    pub fn new(w: i32, h: i32) -> Level {
        Level {
            w,
            h,
            tiles: vec![Vec3::new(0.0, 0.0, 0.0);( w*h) as usize],
            tape: vec![Vec3::new(1.0, 0.0, 0.0)],
            player: (0,0),
            goal: (0,0),
            powerups: vec![],
            alive: true,
            tape_cursor: 0,
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

        self.tiles[(x * self.h + y) as usize]
    }

    pub fn resize(&mut self, new_w: i32, new_h: i32) {
        if new_w < 1 || new_h < 1 {
            return;
        }

        let mut new_tiles = vec![Vec3::new(0.0, 0.0, 0.0); (new_w*new_h) as usize];
        for i in 0..new_w.min(self.w) {
            for j in 0..new_h.min(self.h) {
                let old_tile = self.tiles[(i*self.h + j) as usize];
                let idx = (i*new_h + j) as usize;
                new_tiles[idx] = old_tile;
            }
        }
        self.w = new_w;
        self.h = new_h;
        self.tiles = new_tiles;
    }

    pub fn draw(&self, buf: &mut TriangleBuffer, buf_uv: &mut TriangleBufferUV, rect: Rect) {
        buf.draw_rect(rect, Vec3::new(0.2, 0.2, 0.2), 2.0);
        for i in 0..self.w {
            for j in 0..self.h {
                buf.draw_rect(rect.dilate(-0.005).grid_child(i, j, self.w, self.h).dilate(-0.005), self.get_tile(i, j), 100.0)
            }
        }
        buf_uv.draw_sprite(rect.grid_child(self.goal.0, self.goal.1, self.w, self.h), 2, 200.0);
        buf_uv.draw_sprite(rect.grid_child(self.player.0, self.player.1, self.w, self.h), if self.alive {0} else {3}, 201.0);
        for powerup in self.powerups.iter() {
            buf_uv.draw_sprite(rect.grid_child(powerup.0, powerup.1, self.w, self.h), 1, 200.0);
        }
    }
}