use crate::level::*;
use crate::kmath::*;
use crate::renderer::*;
use crate::application::*;
use crate::game::*;
use crate::level_menu::*;
use crate::manifest::*;
use crate::kgui::*;
use crate::rendererUV::TriangleBufferUV;

use std::collections::HashMap;
use std::fmt::*;

use glutin::event::VirtualKeyCode;


#[derive(Clone, Copy, Debug)]
pub enum EditorCommand {
    PlaceTile(i32, i32),
    ClearTile(i32, i32),
    PickTile(i32, i32),

    SelectTileWedge(i32),
    RotateRight,
    RotateLeft,

    AlterDims(i32, i32),
    
    AddPaletteTile,
    PlacePaletteTile(i32),
    PickPaletteTile(i32),
    RemovePaletteTile(i32),

    PlayLevel,
    SaveLevel,
    LoadLevel,
}


pub struct Button {
    rect: Rect,
    command: EditorCommand,
    hotkey: VirtualKeyCode,
    appearance: ButtonAppearance,
}

pub enum ButtonAppearance {
    Colour(Vec3),
    Texture(i32),
}

pub struct Editor {
    pub level: Level,
    pub place_tile: Tile,
    pub tile_selection: usize,
}

impl Scene for Editor {
    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let click = inputs.events.iter().any(|e| match e {KEvent::MouseLeft(true) => true, _ => false});
        let clickr = inputs.events.iter().any(|e| match e {KEvent::MouseRight(true) => true, _ => false});
        let clickm = inputs.events.iter().any(|e| match e {KEvent::MouseMiddle(true) => true, _ => false});
        let mut so = SceneOutcome::None;
        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);

        let center_pane = inputs.screen_rect.child(0.15, 0.0, 0.7, 1.0);
        let left_pane = Rect::new(0.0, 0.0, (inputs.screen_rect.w - center_pane.w) / 2.0, 1.0);
        let mut button = |x, y, cmd, icon| {
            let button_rect = left_pane.grid_child(x, y, 2, 5).dilate(-0.01);
            buf.draw_rect(button_rect, Vec3::new(0.1, 0.1, 0.1), 5.0);
            buf_uv.draw_sprite(button_rect.fit_center_square(), icon, 6.0);
            if click && button_rect.contains(inputs.mouse_pos) {
                self.handle_command(cmd)
            } else {
                SceneOutcome::None
            }
        };
        match so { SceneOutcome::None => {so = button(0, 0, EditorCommand::PlayLevel, PLAY) }, _ => {}};
        match so { SceneOutcome::None => {so = button(1, 0, EditorCommand::LoadLevel, OPEN) }, _ => {}};
        button(0, 1, EditorCommand::SaveLevel, SAVE);
        button(0, 2, EditorCommand::AlterDims(1, 0), PLUS_W);
        button(1, 2, EditorCommand::AlterDims(-1, 0), MINUS_W);
        button(0, 3, EditorCommand::AlterDims(0, 1), PLUS_H);
        button(1, 3, EditorCommand::AlterDims(0, -1), MINUS_H);
        button(0, 4, EditorCommand::AddPaletteTile, PLUS_TAPE);

        let (maybe_rollover_palette, maybe_rollover_grid) = self.level.frame(&mut buf, &mut buf_uv, center_pane, &inputs, None);
        if let Some(rollover_palette) = maybe_rollover_palette {
            if click || inputs.held_lmb {
                self.handle_command(EditorCommand::PlacePaletteTile(rollover_palette));
            } else if clickr {
                self.handle_command(EditorCommand::RemovePaletteTile(rollover_palette));
            } else if clickm || inputs.held_mmb {
                self.handle_command(EditorCommand::PickPaletteTile(rollover_palette));
            }
        }
        if let Some((x, y)) = maybe_rollover_grid {
            if click || inputs.held_lmb {
                self.handle_command(EditorCommand::PlaceTile(x, y));
            } else if clickr || inputs.held_rmb {
                self.handle_command(EditorCommand::ClearTile(x, y));
            } else if clickm || inputs.held_mmb {
                self.handle_command(EditorCommand::PickTile(x, y));
            }
        }

        let mut scene_outcomes: Vec<SceneOutcome> = inputs.events.iter().filter_map(|e| match e {
            KEvent::Keyboard(VirtualKeyCode::Q, true) => Some(EditorCommand::RotateLeft),
            KEvent::Keyboard(VirtualKeyCode::E, true) => Some(EditorCommand::RotateRight),
            KEvent::Keyboard(VirtualKeyCode::Space, true) => Some(EditorCommand::PlayLevel),
            KEvent::Keyboard(VirtualKeyCode::O, true) => Some(EditorCommand::LoadLevel),
            KEvent::Keyboard(VirtualKeyCode::S, true) => Some(EditorCommand::SaveLevel),
            _ => None,
        }).map(|c| self.handle_command(c)).collect();
        scene_outcomes.push(so);

        let right_pane = Rect::new(center_pane.right(), 0.0, (inputs.screen_rect.w - center_pane.w) / 2.0, 1.0);

        let place_tile_pane = Rect::new(right_pane.x, 0.0, right_pane.w, right_pane.w);
        let place_tile_square = place_tile_pane.dilate(-0.01);

        for i in 0..=3 {
            let place_tri = place_tile_square.tri_child(i);
            if click && place_tri.contains(inputs.mouse_pos) {
                self.tile_selection = i;
                println!("spaget {}", i);
            }
            if self.tile_selection == i {
                buf.draw_tri(place_tri.dilate(0.05), Vec3::new(1.0, 1.0, 1.0), 10.0);
                buf.draw_tri(place_tri, COLOURS[self.place_tile[i] as usize], 11.0);
            } else {
                buf.draw_tri(place_tri, COLOURS[self.place_tile[i] as usize], 9.0);
            }
        }

        let right_bot_pane = Rect::new(place_tile_pane.x, place_tile_pane.bot(), place_tile_pane.w, right_pane.h - place_tile_pane.h);
        for i in 0..COLOURS.len() {
            let colour_rect = right_bot_pane.grid_child(0, i as i32, 1, COLOURS.len() as i32);
            buf.draw_rect(colour_rect, COLOURS[i], 10.0);
            if click && colour_rect.contains(inputs.mouse_pos) {
                self.place_tile[self.tile_selection] = i as u8;
            }
        }

        buf.draw_rect(left_pane, Vec3::new(0.2, 0.2, 0.2), 1.0);
        buf.draw_rect(right_pane, Vec3::new(0.2, 0.2, 0.2), 1.0);

        (scene_outcomes.remove(0), buf, Some(buf_uv))
    }

    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        match signal {
            SceneSignal::LevelChoice(level) => {
                self.level = level;
                return SceneOutcome::None
            },
            _ => {SceneOutcome::None},
        }
    }
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            level: Level::new(4,4),
            place_tile: [0; 4],
            tile_selection: 0,
        }
    }

    pub fn handle_command(&mut self, command: EditorCommand) -> SceneOutcome {
        println!("Editor Command: {:?}", command);
        match command {
            EditorCommand::AddPaletteTile => {self.level.tile_palette.push(self.place_tile)},
            EditorCommand::PlacePaletteTile(i) => {self.level.tile_palette[i as usize] = self.place_tile},
            EditorCommand::RemovePaletteTile(i) => {
                if self.level.tile_palette.len() > 1 {
                    self.level.tile_palette.remove(i as usize);
                }
            },
            EditorCommand::PickPaletteTile(i) => {self.place_tile = self.level.tile_palette[i as usize]},

            EditorCommand::RotateLeft => {self.place_tile = [self.place_tile[1], self.place_tile[2], self.place_tile[3], self.place_tile[0]]},
            EditorCommand::RotateRight => {self.place_tile = [self.place_tile[3], self.place_tile[0], self.place_tile[1], self.place_tile[2]]},

            EditorCommand::PickTile(x, y) => {
                if let Some(pick_tile) = self.level.get_tile(x, y) {
                    self.place_tile = pick_tile;
                }
            },
            EditorCommand::ClearTile(x, y) => {
                self.level.clear_tile(x, y);
                self.level.set_locked(x, y, false);
            },
            EditorCommand::PlaceTile(x, y) => {
                self.level.set_tile(x, y, self.place_tile);
                self.level.set_locked(x, y, true);
            },
            EditorCommand::SelectTileWedge(i) => {self.tile_selection = i as usize},

            EditorCommand::AlterDims(dx, dy) => {
                self.level.resize(self.level.w + dx, self.level.h + dy);
            },

            EditorCommand::PlayLevel => {return SceneOutcome::Push(Box::new(Game {level: self.level.clone(), place_tile: self.level.tile_palette[0], place_idx: 0}))},
            EditorCommand::SaveLevel => {
                let hash = self.level.hash();
                let path = format!("levels/{}.level", hash);
                let metadata = LevelMetadata {
                    level: self.level.clone(), name: String::from("untitled"), rating: 69,
                };
                metadata.save(&path);
            },
            EditorCommand::LoadLevel => {return SceneOutcome::Push(Box::new(LevelMenu::new()))},
        }
        return SceneOutcome::None;
    }
}
