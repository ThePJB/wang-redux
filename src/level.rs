use crate::kmath::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::rect::*;
use crate::manifest::*;

use std::fs::File;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Tile {
    Colour(usize),
    Wild,
    Wall,
}

#[derive(Serialize, Deserialize)]
pub struct LevelMetadata {
    pub level: Level,
    pub name: String,
    pub rating: i32,
}

impl LevelMetadata {
    pub fn save(&self, path: &str) {
        let str = serde_json::to_string(self).unwrap();
        let mut f = File::create(path);
        if f.is_ok() {
            f.unwrap().write_all(str.as_bytes());
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Level {
    tiles: Vec<Tile>,
    pub w: i32,
    pub h: i32,

    pub tape: Vec<usize>,
    pub tape_cursor: i32,
    pub player: (i32, i32),
    pub alive: bool,
    pub goal: (i32, i32),
    pub powerups: Vec<(i32, i32, i32)>,
    pub gotos: Vec<(i32, i32, i32)>,
}

impl Level {
    pub fn hash(&self) -> u32 {
        let mut h = 0u32;
        h += khash(self.w as u32);
        h = khash(h);
        h += khash(self.h as u32);
        h = khash(h);
        h += khash(self.player.0 as u32);
        h = khash(h);
        h += khash(self.player.1 as u32);
        h = khash(h);
        h += khash(self.goal.0 as u32);
        h = khash(h);
        h += khash(self.goal.1 as u32);
        h = khash(h);

        for (x, y, amount) in self.powerups.iter() {
            h += khash(*x as u32);
            h = khash(h);
            h += khash(*y as u32);
            h = khash(h);
            h += khash(*amount as u32);
            h = khash(h);
        }

        for (x, y, amount) in self.gotos.iter() {
            h += khash(*x as u32);
            h = khash(h);
            h += khash(*y as u32);
            h = khash(h);
            h += khash(*amount as u32);
            h = khash(h);
        }

        for c in self.tape.iter() {
            h += khash(*c as u32);
            h = khash(h);
        }

        for tile in self.tiles.iter() {
            match tile {
                Tile::Colour(c) => {
                    h += khash(*c as u32);
                    h = khash(h);
                },
                Tile::Wall => {
                    h += khash(999);
                    h = khash(h);
                },
                Tile::Wild => {
                    h += khash(666);
                    h = khash(h);
                },
            }
        }
        h
    }

    pub fn complexity(&self) -> u32 {
        let mut complexity = 0u32;

        let num_tape_colours = {
            let mut tape_sort = self.tape.clone();
            tape_sort.sort();
            tape_sort.dedup();
            tape_sort.iter().count()
        };

        let num_level_colours = {
            let mut level_sort: Vec<usize> = self.tiles.iter().filter_map(|t| match t {Tile::Colour(c) => Some(*c), _ => None}).collect();
            level_sort.sort();
            level_sort.dedup();
            level_sort.iter().count()
        };

        let any_gotos = if self.gotos.len() == 0 { 0 } else { 1 } as usize;
        let any_walls = if self.tiles.iter().filter(|t| match t {Tile::Wall => true, _ => false}).count() == 0 { 0 } else { 1 } as usize;
        let any_wilds = if self.tiles.iter().filter(|t| match t {Tile::Wild => true, _ => false}).count() == 0 { 0 } else { 1 } as usize;

        let max_powerup_size = {
            let mut powerups_sort = self.powerups.clone();
            powerups_sort.sort_by_key(|(x, y, n)| *n);
            powerups_sort.iter().map(|(x, y, n)| *n).nth(0).unwrap_or(0)
        } as usize;

        // bigger: comes later

        (1 * self.tiles.len() + 
        100 * self.tape.len() +
        1000 * num_level_colours +
        10000 * num_tape_colours + 
        100000 * any_walls +
        1000000 * any_wilds +
        10000000 * max_powerup_size +
        100000000 * any_gotos +

        10000 * self.gotos.len() +
        10000 * self.powerups.len()) as u32
    }

    pub fn new(w: i32, h: i32) -> Level {
        Level {
            w,
            h,
            tiles: vec![Tile::Colour(0);( w*h) as usize],
            tape: vec![0],
            player: (0,0),
            goal: (0,0),
            powerups: vec![],
            gotos: vec![],
            alive: true,
            tape_cursor: 0,
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("set tile out of bounds");
        }

        self.tiles[(x * self.h + y) as usize] = tile;
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("get tile out of bounds");
        }

        self.tiles[(x * self.h + y) as usize]
    }

    pub fn resize(&mut self, new_w: i32, new_h: i32) {
        if new_w < 1 || new_h < 1 {
            return;
        }

        let mut new_tiles = vec![Tile::Colour(0); (new_w*new_h) as usize];
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
                match self.get_tile(i, j) {
                    Tile::Colour(colour) => {buf.draw_rect(rect.dilate(-0.005).grid_child(i, j, self.w, self.h).dilate(-0.005), COLOURS[colour], 100.0)},
                    _ => {},
                }
                
            }
        }
        buf_uv.draw_sprite(rect.grid_child(self.goal.0, self.goal.1, self.w, self.h), 2, 200.0);
        for powerup in self.powerups.iter() {
            buf_uv.draw_sprite(rect.grid_child(powerup.0, powerup.1, self.w, self.h), 1, 201.0);
        }
        buf_uv.draw_sprite(rect.grid_child(self.player.0, self.player.1, self.w, self.h), if self.alive {0} else {3}, 202.0);
    }
}