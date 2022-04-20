use crate::kgui::*;
use crate::kmath::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::manifest::*;

use std::fs::File;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

pub type Tile = [u8;4];

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
    pub tiles: Vec<Option<Tile>>,
    pub locked: Vec<bool>,
    pub w: i32,
    pub h: i32,

    pub tile_palette: Vec<Tile>,
}

impl Level {
    pub fn hash(&self) -> u32 {
        let mut h = 0u32;
        
        for tile in self.tiles.iter() {
            match tile {
                Some(colours) => {
                    for colour in colours {
                        h += khash(*colour as u32);
                        h = khash(h);
                    }
                },
                None => {
                    h += khash(666);
                    h = khash(h);
                },
            }
        }
        
        h
    }

    pub fn complexity(&self) -> u32 {
        //first total num tiles then unique tiles then num colours
        let mut complexity = 0u32;

        let num_tiles = self.w * self.h;
        let num_palette_tiles = self.tile_palette.len();
        let num_colours = {
            let mut colours: Vec<u8> = self.tile_palette.iter().flatten().map(|x| *x).collect();
            colours.sort();
            colours.dedup();
            colours.iter().count()
        };

        num_colours as u32 * 10000 + num_palette_tiles as u32 * 100 + num_tiles as u32 
    }

    pub fn new(w: i32, h: i32) -> Level {
        Level {
            w,
            h,
            tiles: vec![None;(w*h) as usize],
            locked: vec![false;(w*h) as usize],
            tile_palette: vec![[0,0,0,0]],
        }
    }

    pub fn can_place(&self, x: i32, y: i32, place_tile: Tile) -> bool {
        if x != 0 {
            if let Some(neigh) = self.get_tile(x, y) {
                if neigh[1] != place_tile[3] {
                    println!("reject left edge neighbour");
                    return false;
                }
            }
        }
    
        if x != self.w - 1 {
            if let Some(neigh) = self.get_tile(x + 1, y) {
                if neigh[3] != place_tile[1] {
                    println!("reject right edge neighbour");
                    return false;
                }
            }
        }
    
        if y != self.h - 1 {
            if let Some(neigh) = self.get_tile(x, y + 1) {
                if neigh[0] != place_tile[2] {
                    println!("reject bottom edge neighbour");
                    return false;
                }
            }
        }
        
        if y != 0 {
            if let Some(neigh) = self.get_tile(x, y - 1) {
                if neigh[2] != place_tile[0] {
                    println!("reject top edge neighbour");
                    return false;
                }
            }
        }
        true
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("set tile out of bounds");
        }

        self.tiles[(x * self.h + y) as usize] = Some(tile);
    }
    pub fn clear_tile(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("set tile out of bounds");
        }

        self.tiles[(x * self.h + y) as usize] = None;
    }

    pub fn set_locked(&mut self, x: i32, y: i32, locked: bool) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("set locked tile out of bounds");
        }
        self.locked[(x * self.h + y) as usize] = locked;
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("get tile out of bounds");
        }

        self.tiles[(x * self.h + y) as usize]
    }

    pub fn get_locked(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            panic!("get tile out of bounds");
        }

        self.locked[(x * self.h + y) as usize]
    }

    pub fn resize(&mut self, new_w: i32, new_h: i32) {
        if new_w < 1 || new_h < 1 {
            return;
        }

        let mut new_tiles = vec![None; (new_w*new_h) as usize];
        let mut new_locked = vec![false; (new_w*new_h) as usize];
        for i in 0..new_w.min(self.w) {
            for j in 0..new_h.min(self.h) {
                let old_tile = self.tiles[(i*self.h + j) as usize];
                let idx = (i*new_h + j) as usize;
                new_tiles[idx] = old_tile;
                let old_locked = self.locked[(i*self.h + j) as usize];
                let idx = (i*new_h + j) as usize;
                new_locked[idx] = old_locked;
            }
        }
        self.w = new_w;
        self.h = new_h;
        self.tiles = new_tiles;
        self.locked = new_locked;
    }

    pub fn frame(&self, buf: &mut TriangleBuffer, buf_uv: &mut TriangleBufferUV,  rect: Rect, inputs: &FrameInputState, selected_tile: Option<i32>) -> (Option<i32>, Option<(i32, i32)>) {
        let tiles_pane = rect.child(0.0, 0.0, 0.2, 1.0);
        let level_pane = rect.child(0.2, 0.0, 0.8, 1.0).fit_aspect_ratio(self.w as f32 / self.h as f32);
        let level_rect = level_pane.dilate(-0.005);

        buf.draw_rect(level_pane, Vec3::new(0.2, 0.2, 0.2), 1.0);

        // buf.draw_rect(tiles_pane, Vec3::new(0.0, 1.0, 0.0), 100.0);
        // buf.draw_rect(level_pane, Vec3::new(1.0, 1.0, 0.0), 100.0);

        let mut select_palette_tile = None;
        let mut select_grid_tile = None;

        for (i, tile) in self.tile_palette.iter().enumerate() {
            let tile_rect = tiles_pane.grid_child(0, i as i32, 1, self.tile_palette.len() as i32).dilate(-0.01).fit_center_square();
            if tile_rect.contains(inputs.mouse_pos) {
                select_palette_tile = Some(i as i32);
            }
            draw_tile(buf, tile_rect, *tile);
            if let Some(idx) = selected_tile {
                if idx as usize == i {
                    buf.draw_rect(tile_rect.dilate(0.01), Vec3::new(1.0, 1.0, 1.0), 2.0);
                }
            }
        }

        for i in 0..self.w {
            for j in 0..self.h {
                let tile_rect = level_rect.grid_child(i, j, self.w, self.h).dilate(-0.005);
                if tile_rect.contains(inputs.mouse_pos) {
                    select_grid_tile = Some((i, j));
                }
                if let Some(colours) = self.get_tile(i, j) {
                    draw_tile(buf, tile_rect, colours);
                    if !self.get_locked(i, j) {
                        buf_uv.draw_sprite(tile_rect, TILE_EDGES, 4.0);
                    }
                } else {
                    buf.draw_rect(tile_rect, Vec3::new(0.15, 0.15, 0.15), 3.0);
                }
            }
        }

        (select_palette_tile, select_grid_tile)
    }
}

pub fn draw_tile(buf: &mut TriangleBuffer, rect: Rect, tile: Tile) {
    for (x, colour) in tile.iter().enumerate() {
        buf.draw_tri(rect.tri_child(x), COLOURS[*colour as usize], 3.0);
    }
}